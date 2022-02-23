using Jacobi.Vst.Core;
using Jacobi.Vst.Host.Interop;
using Serilog;
using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace PhotoConsequences
{
    class ImageProcessor
    {
        public InsertedPlugin ChainedPlugin { get; set; }

        public ImageProcessor(InsertedPlugin ctx)
        {
            ChainedPlugin = ctx;
        }

        private Color ColorFromHSV(double hue, double saturation, double value)
        {
            int hi = Convert.ToInt32(Math.Floor(hue / 60)) % 6;
            double f = hue / 60 - Math.Floor(hue / 60);

            value = value * 255;
            int v = Convert.ToInt32(value);
            int p = Convert.ToInt32(value * (1 - saturation));
            int q = Convert.ToInt32(value * (1 - f * saturation));
            int t = Convert.ToInt32(value * (1 - (1 - f) * saturation));

            if (hi == 0)
                return Color.FromArgb(255, v, t, p);
            else if (hi == 1)
                return Color.FromArgb(255, q, v, p);
            else if (hi == 2)
                return Color.FromArgb(255, p, v, t);
            else if (hi == 3)
                return Color.FromArgb(255, p, q, v);
            else if (hi == 4)
                return Color.FromArgb(255, t, p, v);
            else
                return Color.FromArgb(255, v, p, q);
        }


        private void ColorToHSV(Color color, out double hue, out double saturation, out double value)
        {
            int max = Math.Max(color.R, Math.Max(color.G, color.B));
            int min = Math.Min(color.R, Math.Min(color.G, color.B));

            hue = color.GetHue();
            saturation = (max == 0) ? 0 : 1d - (1d * min / max);
            value = max / 255d;
        }

        /// <summary>
        /// Processes image
        /// </summary>
        /// <param name="Image">Input Image</param>
        /// <returns>
        /// Output image
        /// </returns>
        public Bitmap ProcessImage(Bitmap image)
        {
            Bitmap outputImage = (Bitmap)image.Clone();
            GraphicsUnit units = GraphicsUnit.Pixel;
            RectangleF size = outputImage.GetBounds(ref units);

            int inputCount = ChainedPlugin.PluginContext.PluginInfo.AudioInputCount;
            int outputCount = ChainedPlugin.PluginContext.PluginInfo.AudioOutputCount;
            int blockSize = (int)(size.Width * size.Height);

            // wrap these in using statements to automatically call Dispose and cleanup the unmanaged memory.
            using VstAudioBufferManager inputMgr = new VstAudioBufferManager(inputCount, blockSize);
            using VstAudioBufferManager outputMgr = new VstAudioBufferManager(outputCount, blockSize);

            foreach (VstAudioBuffer buffer in inputMgr.Buffers)
            {
                var span = buffer.AsSpan();

                for (int x = 0; x < size.Width; x++)
                {
                    for (int y = 0; y < size.Height; y++)
                    {

                        switch (ChainedPlugin.ImageProcessingInput)
                        {
                            case Channel.Hue:
                                span[(int)size.Width * y + x] = outputImage.GetPixel(x, y).GetHue();
                                break;
                            case Channel.Saturation:
                                span[(int)size.Width * y + x] = outputImage.GetPixel(x, y).GetSaturation();
                                break;
                            case Channel.Value:
                                span[(int)size.Width * y + x] = outputImage.GetPixel(x, y).GetBrightness();
                                break;
                            default:
                                break;
                        }
                    }
                }
            }

            ChainedPlugin.PluginContext.PluginCommandStub.Commands.SetBlockSize(blockSize);
            ChainedPlugin.PluginContext.PluginCommandStub.Commands.SetSampleRate(ChainedPlugin.SampleRate);

            VstAudioBuffer[] inputBuffers = inputMgr.Buffers.ToArray();
            VstAudioBuffer[] outputBuffers = outputMgr.Buffers.ToArray();

            ChainedPlugin.PluginContext.PluginCommandStub.Commands.MainsChanged(true);
            ChainedPlugin.PluginContext.PluginCommandStub.Commands.StartProcess();
            ChainedPlugin.PluginContext.PluginCommandStub.Commands.ProcessReplacing(inputBuffers, outputBuffers);
            ChainedPlugin.PluginContext.PluginCommandStub.Commands.StopProcess();
            ChainedPlugin.PluginContext.PluginCommandStub.Commands.MainsChanged(false);

            for (int x = 0; x < size.Width; x++)
            {
                for (int y = 0; y < size.Height; y++)
                {
                    var pixel = outputImage.GetPixel(x, y);
                    ColorToHSV(pixel, out var hue, out var saturation, out var value);
                    var processingBuffer = (int)ChainedPlugin.AudioProcessingOuput;


                    switch (ChainedPlugin.ImageProcessingInput)
                    {
                        case Channel.Hue:
                            hue = (float)Math.Clamp(outputBuffers[processingBuffer][(int)size.Width * y + x], 0.0, 360.0) * ChainedPlugin.Wet;
                            break;
                        case Channel.Saturation:
                            saturation = (float)Math.Clamp(outputBuffers[processingBuffer][(int)size.Width * y + x], 0.0, 1.0) * ChainedPlugin.Wet;
                            break;
                        case Channel.Value:
                            value = (float)Math.Clamp(outputBuffers[processingBuffer][(int)size.Width * y + x], 0.0, 1.0) * ChainedPlugin.Wet;
                            break;
                        default:
                            break;
                    }

                    outputImage.SetPixel(x, y, ColorFromHSV(hue, saturation, value));
                }
            }

            Log.Information("Done!");

            return outputImage;
        }
    }
}
