using Jacobi.Vst.Host.Interop;
using Serilog;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace VSTImage
{
    public partial class MainForm : Form
    {
        private List<PluginChain> _plugins = new List<PluginChain>();
        private List<Bitmap> Images = new List<Bitmap>();
        private ProcessingProgress ProcessingProgress;

        public MainForm()
        {
            InitializeComponent();
            SetImageControls();
        }

        private void ReleaseAllPlugins()
        {
            foreach (var ctx in _plugins)
            {
                // dispose of all (unmanaged) resources
                ctx.PluginContext.Dispose();
            }

            _plugins.Clear();
        }

        private void SetImageControls()
        {
            if (Images.Any() && Images.Last() != null)
            {
                toolApplyBtn.Enabled = true;
                toolSaveimgBtn.Enabled = true;
                pictureBox.Image = Images.Last();
            }
            else
            {
                toolApplyBtn.Enabled = false;
                toolSaveimgBtn.Enabled = false;
            }
        }

        private void SetPluginControls()
        {
            if (listPlugins.SelectedItems.Count > 0)
            {
                foreach (Control item in groupBoxOptions.Controls)
                {
                    item.Enabled = true;
                }

                var selection = _plugins[listPlugins.SelectedItems[0].Index];

                hBox.SelectedIndex = (int)selection.ProcessingValues[Channel.Hue];
                sBox.SelectedIndex = (int)selection.ProcessingValues[Channel.Saturation];
                vBox.SelectedIndex = (int)selection.ProcessingValues[Channel.Value];
                inputBox.SelectedIndex = (int)selection.Input;
            }
            else
            {
                foreach (Control item in groupBoxOptions.Controls)
                {
                    item.Enabled = false;
                }
            }
        }

        private ProcessorArchitecture GetArch()
        {
            Assembly currentAssem = Assembly.GetExecutingAssembly();
            return currentAssem.GetName().ProcessorArchitecture;
        }

        private void FillPluginList()
        {
            listPlugins.Items.Clear();

            foreach (var ctx in _plugins)
            {
                ListViewItem lvItem = new ListViewItem(ctx.PluginContext.PluginCommandStub.Commands.GetEffectName());
                lvItem.SubItems.Add(ctx.PluginContext.PluginCommandStub.Commands.GetProductString());
                lvItem.SubItems.Add(ctx.PluginContext.PluginCommandStub.Commands.GetVendorString());
                lvItem.SubItems.Add(ctx.PluginContext.PluginCommandStub.Commands.GetVendorVersion().ToString());
                lvItem.SubItems.Add(ctx.PluginContext.Find<string>("PluginPath"));
                lvItem.Tag = ctx;

                listPlugins.Items.Add(lvItem);
            }

            openPluginEditorBtn.Enabled = _plugins.Any();
        }

        private void ShowEditor(VstPluginContext PluginContext)
        {
            EditorFrame dlg = new EditorFrame
            {
                PluginCommandStub = PluginContext.PluginCommandStub
            };

            PluginContext.PluginCommandStub.Commands.MainsChanged(true);
            dlg.ShowDialog(this);
            PluginContext.PluginCommandStub.Commands.MainsChanged(false);
        }

        private void Form1_Load(object sender, EventArgs e)
        {

        }

        private void toolAddVstBtn_Click(object sender, EventArgs e)
        {
            openFileDlg.Filter = $"VST {GetArch()} 2.4 Plugins(*.dll)|*.dll";

            if (openFileDlg.ShowDialog(this) == DialogResult.OK)
            {
                try
                {
                    var plugin = new PluginChain(openFileDlg.FileName);

                    if (plugin.PluginContext != null)
                    {
                        _plugins.Add(plugin);
                        FillPluginList();
                    }
                    else
                    {
                        MessageBox.Show(this, "Failed to create plugin context", openFileDlg.FileName, MessageBoxButtons.OK, MessageBoxIcon.Error);
                    }
                }
                catch (BadImageFormatException ex)
                {
                    Log.Error("Failed to open VST: {0}", ex.ToString());
                    MessageBox.Show(this, $"This VSTImage build can open only {GetArch()} plugin.", "Plugin load error", MessageBoxButtons.OK, MessageBoxIcon.Error);
                }
                catch (EntryPointNotFoundException ex)
                {
                    Log.Error("Failed to open VST: {0}", ex.ToString());
                    MessageBox.Show(this, $"This dll file is not a VST Plugin.", "Plugin load error", MessageBoxButtons.OK, MessageBoxIcon.Error);
                }
                catch (Exception ex)
                {
                    Log.Error("Failed to open VST: {0}", ex.ToString());
                    MessageBox.Show(this, e.ToString(), "Plugin load error", MessageBoxButtons.OK, MessageBoxIcon.Error);
                }
            }
        }

        private void openPluginEditorBtn_Click(object sender, EventArgs e)
        {
            ShowEditor(_plugins[listPlugins.SelectedItems[0].Index].PluginContext);
        }

        private void toolApplyBtn_Click(object sender, EventArgs e)
        {
            if (backgroundWorker.IsBusy != true)
            {
                Images.RemoveRange(1, Images.Count - 1);
                backgroundWorker.RunWorkerAsync();
                foreach (ToolStripItem item in toolStrip1.Items)
                {
                    Log.Verbose("{0}", item.Name);
                    item.Enabled = false;
                }
                ProcessingProgress = new ProcessingProgress();
                ProcessingProgress.Show();
            }
        }

        private void toolOpenImageBtn_Click(object sender, EventArgs e)
        {
            openFileDlg.Filter = "Image Files(*.BMP;*.JPG;*.GIF,*.TIFF,*.PNG)|*.BMP;*.JPG;*.GIF;*.TIFF;*.PNG";

            if (openFileDlg.ShowDialog(this) == DialogResult.OK)
            {
                try
                {
                    var img = new Bitmap(openFileDlg.FileName);

                    Log.Information("Opened a image with: {0} pixel format", img.PixelFormat);

                    if (img.PixelFormat is System.Drawing.Imaging.PixelFormat.Indexed or System.Drawing.Imaging.PixelFormat.Format1bppIndexed or
                        System.Drawing.Imaging.PixelFormat.Format4bppIndexed or System.Drawing.Imaging.PixelFormat.Format8bppIndexed)
                    {
                        MessageBox.Show(this, "This image have a indexed pixel format - processing unsupported", openFileDlg.FileName, MessageBoxButtons.OK, MessageBoxIcon.Error);
                        return;
                    }

                    Images.Clear();
                    Images.Add(img);
                }
                catch (ArgumentException)
                {
                    MessageBox.Show(this, "This is not valid image format", openFileDlg.FileName, MessageBoxButtons.OK, MessageBoxIcon.Error);
                }


                SetImageControls();
            }
        }

        private void Form1_FormClosed(object sender, FormClosedEventArgs e)
        {
            ReleaseAllPlugins();
            foreach (var item in Images)
            {
                item.Dispose();
            }
            Images.Clear();
        }

        private void listPlugins_SelectedIndexChanged(object sender, EventArgs e)
        {
            SetPluginControls();
        }

        private void removePlugBtn_Click(object sender, EventArgs e)
        {
            var selected = listPlugins.SelectedItems[0].Index;
            var res = MessageBox.Show(this, "Remove plugin?", _plugins[selected].PluginContext.PluginCommandStub.Commands.GetEffectName(), MessageBoxButtons.YesNo, MessageBoxIcon.Warning);
            if (res == DialogResult.Yes)
            {
                if (_plugins[selected].PluginContext != null) { _plugins[selected].PluginContext.Dispose(); }
                _plugins.RemoveAt(selected);
                FillPluginList();
                SetPluginControls();
            }
        }

        private void hBox_SelectedIndexChanged(object sender, EventArgs e)
        {
            var selected = listPlugins.SelectedItems[0].Index;
            _plugins[selected].ProcessingValues[Channel.Hue] = (Processing)hBox.SelectedIndex;
        }

        private void sBox_SelectedIndexChanged(object sender, EventArgs e)
        {
            var selected = listPlugins.SelectedItems[0].Index;
            _plugins[selected].ProcessingValues[Channel.Saturation] = (Processing)sBox.SelectedIndex;
        }

        private void vBox_SelectedIndexChanged(object sender, EventArgs e)
        {
            var selected = listPlugins.SelectedItems[0].Index;
            _plugins[selected].ProcessingValues[Channel.Value] = (Processing)vBox.SelectedIndex;
        }

        private void trackWet_ValueChanged(object sender, EventArgs e)
        {
            var selected = listPlugins.SelectedItems[0].Index;
            _plugins[selected].Dry = trackWet.Value / 100; 
        }

        private void inputBox_SelectedIndexChanged(object sender, EventArgs e)
        {
            var selected = listPlugins.SelectedItems[0].Index;
            _plugins[selected].Input = (Channel)vBox.SelectedIndex;
        }

        private void backgroundWorker_DoWork(object sender, DoWorkEventArgs e)
        {
            var complete = 0;

            foreach (var plugin in _plugins)
            {
                Log.Information("Proccessing image...");
                ImageProcessor processor = new ImageProcessor(plugin, (float)sampleRateInput.Value);
                var image = processor.ProcessImage(Images.Last());
                Images.Add(image);
                complete++;
                backgroundWorker.ReportProgress(complete);
            }
        }

        private void backgroundWorker_ProgressChanged(object sender, ProgressChangedEventArgs e)
        {
            ProcessingProgress.UpdateProgress($"Effects processed: {e.ProgressPercentage} of {_plugins.Count}");
        }

        private void backgroundWorker_RunWorkerCompleted(object sender, RunWorkerCompletedEventArgs e)
        {
            SetImageControls();
            foreach (ToolStripItem item in toolStrip1.Items)
            {
                Log.Verbose("{0}", item.Name);
                item.Enabled = true;
            }
            ProcessingProgress.Close();
            ProcessingProgress.Dispose();
        }

        private void undoBtn_Click(object sender, EventArgs e)
        {
            Log.Verbose("Undo buffer: {0}", Images.Count);
            if (Images.Count > 1)
            {
                Images.RemoveAt(Images.Count - 1);
            }

            SetImageControls();
        }

        private void toolSaveimgBtn_Click(object sender, EventArgs e)
        {
            saveFileDlg.Filter = "Bitmap file(*.BMP)|*.BMP|Portable Network Graphics(*.PNG)|*.PNG|JPEG(*.JPEG)|*.jpg";

            if (saveFileDlg.ShowDialog(this) == DialogResult.OK)
            {
                try
                {
                    Images.Last().Save(saveFileDlg.FileName);
                }
                catch (Exception error)
                {
                    MessageBox.Show($"Saving error: {error.Message}", "Image saving failed!", MessageBoxButtons.OK);
                }
            }
        }
    }
}
