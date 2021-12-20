using Jacobi.Vst.Host.Interop;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace VSTImage
{
    public enum Processing
    {
        Left = 0,
        Right = 1,
        None = 2,
    }

    public enum Channel
    {
        Hue,
        Saturation,
        Value,
    }

    class PluginChain
    {
        public VstPluginContext PluginContext { get; set; }
        public Dictionary<Channel, Processing> ProcessingValues { get; set; }
        public float Dry { get; set; }

        public PluginChain(VstPluginContext ctx)
        {
            PluginContext = ctx;
            Dry = 1.0f;
            ProcessingValues = new Dictionary<Channel, Processing>();
            ProcessingValues.Add(Channel.Hue, Processing.Left);
            ProcessingValues.Add(Channel.Saturation, Processing.None);
            ProcessingValues.Add(Channel.Value, Processing.None);
        }
    }
}
