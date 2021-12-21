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
                hBox.Enabled = true;
                vBox.Enabled = true;
                sBox.Enabled = true;
                trackWet.Enabled = true;
                openPluginEditorBtn.Enabled = true;
                removePlugBtn.Enabled = true;

                var selection = _plugins[listPlugins.SelectedItems[0].Index];

                hBox.SelectedIndex = (int)selection.ProcessingValues[Channel.Hue];
                sBox.SelectedIndex = (int)selection.ProcessingValues[Channel.Saturation];
                vBox.SelectedIndex = (int)selection.ProcessingValues[Channel.Value];
            }
            else
            {
                hBox.Enabled = false;
                vBox.Enabled = false;
                sBox.Enabled = false;
                trackWet.Enabled = false;
                openPluginEditorBtn.Enabled = false;
                removePlugBtn.Enabled = false;
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

        private VstPluginContext OpenPlugin(string pluginPath)
        {
            try
            {
                HostCommandStub hostCmdStub = new HostCommandStub();
                hostCmdStub.PluginCalled += new EventHandler<PluginCalledEventArgs>(HostCmdStub_PluginCalled);

                VstPluginContext ctx = VstPluginContext.Create(pluginPath, hostCmdStub);

                // add custom data to the context
                ctx.Set("PluginPath", pluginPath);
                ctx.Set("HostCmdStub", hostCmdStub);

                // actually open the plugin itself
                ctx.PluginCommandStub.Commands.Open();

                return ctx;
            }
            catch (BadImageFormatException e)
            {
                Log.Error("Failed to open VST: {0}", e.ToString());
                MessageBox.Show(this, $"This VSTImage build can open only {GetArch()} plugin.", $"{pluginPath}: Plugin load error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
            catch (EntryPointNotFoundException e)
            {
                Log.Error("Failed to open VST: {0}", e.ToString());
                MessageBox.Show(this, $"This dll file is not a VST Plugin.", $"{pluginPath}: Plugin load error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
            catch (Exception e)
            {
                Log.Error("Failed to open VST: {0}", e.ToString());
                MessageBox.Show(this, e.ToString(), "Plugin load error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }

            return null;
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

        private VstPluginContext SelectedPluginContext
        {
            get
            {
                if (listPlugins.SelectedItems.Count > 0)
                {
                    return (VstPluginContext)listPlugins.SelectedItems[0].Tag;
                }

                return null;
            }
        }


        private void HostCmdStub_PluginCalled(object sender, PluginCalledEventArgs e)
        {
            HostCommandStub hostCmdStub = (HostCommandStub)sender;

            // can be null when called from inside the plugin main entry point.
            if (hostCmdStub.PluginContext.PluginInfo != null)
            {
                Log.Verbose("Plugin " + hostCmdStub.PluginContext.PluginInfo.PluginID + " called:" + e.Message);
            }
            else
            {
                Log.Verbose("The loading Plugin called:" + e.Message);
            }
        }

        private void Form1_Load(object sender, EventArgs e)
        {

        }

        private void toolAddVstBtn_Click(object sender, EventArgs e)
        {
            openFileDlg.Filter = $"VST {GetArch()} 2.4 Plugins(*.dll)|*.dll";


            if (openFileDlg.ShowDialog(this) == DialogResult.OK)
            {
                VstPluginContext ctx = OpenPlugin(openFileDlg.FileName);

                if (ctx != null)
                {
                    _plugins.Add(new PluginChain(ctx));
                    FillPluginList();
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
                // Start the asynchronous operation.
                backgroundWorker.RunWorkerAsync();
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

        private void backgroundWorker_DoWork(object sender, DoWorkEventArgs e)
        {
            var complete = 0;
            var max = _plugins.Count;

            foreach (var plugin in _plugins)
            {
                Log.Information("Proccessing image...");
                ImageProcessor processor = new ImageProcessor(plugin);
                var image = processor.ProcessImage(Images.Last());
                Images.Add(image);
                complete++;
                backgroundWorker.ReportProgress(complete / max * 100);
            }
        }

        private void backgroundWorker_ProgressChanged(object sender, ProgressChangedEventArgs e)
        {
            progressProcessing.Value = e.ProgressPercentage;
        }

        private void backgroundWorker_RunWorkerCompleted(object sender, RunWorkerCompletedEventArgs e)
        {
            SetImageControls();
        }

        private void undoBtn_Click(object sender, EventArgs e)
        {
            int state = 0;
            foreach (var item in Images)
            {
                item.Save($"undostate{state}.png");
                state++;
            }
            Log.Verbose("Undo buffer: {0}", Images.Count);
            if (Images.Count > 1)
            {
                Images.RemoveAt(Images.Count - 1);
            }

            SetImageControls();
        }
    }
}
