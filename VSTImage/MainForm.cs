using Jacobi.Vst.Host.Interop;
using Newtonsoft.Json;
using Serilog;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Drawing.Imaging;
using System.IO;
using System.IO.Compression;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace VSTImage
{
    public partial class MainForm : Form
    {
        private PluginRack Rack = new PluginRack();
        private List<Bitmap> Images = new List<Bitmap>();
        private ProcessingProgress ProcessingProgress;
        private string SavePath;

        public MainForm(string[] args)
        {
            InitializeComponent();
            SetImageControls();
            Log.Information("Command line: {0}", args);
            if (args.Any() && args[0].TrimEnd().EndsWith(".viproj"))
            {
                LoadProject(args[0]);
            }
        }

        private bool SaveProjUI()
        {
            if (SavePath == null)
            {
                saveFileDlg.Filter = "VSTImage project file(*.viproj)|*.viproj";
                if (saveFileDlg.ShowDialog(this) == DialogResult.OK)
                {
                    SavePath = saveFileDlg.FileName;
                }
                else
                {
                    return false;
                }
            }

            try
            {
                SaveProject(SavePath);
            }
            catch (Exception error)
            {
                MessageBox.Show($"Saving error: {error.Message}", "Project saving failed!", MessageBoxButtons.OK, MessageBoxIcon.Error);
                return false;
            }

            return true;
        }

        private bool SaveConformation()
        {
            if (Images.Any() && Rack.Plugins.Any() && 
                MessageBox.Show($"Save {SavePath ?? "project"} before exiting?", "Project managment", MessageBoxButtons.YesNo, MessageBoxIcon.Warning) == DialogResult.Yes)
            {
                SaveProjUI();
            }
            return false;
        }

        private void SaveProject(string path)
        {
            foreach (var ctx in Rack.Plugins) { ctx.SetState(); }
            if (File.Exists(path)) { File.Delete(path); }

            var pluginState = JsonConvert.SerializeObject(Rack.Plugins);

            using (FileStream zipToOpen = new FileStream(path, FileMode.OpenOrCreate))
            {
                using (ZipArchive archive = new ZipArchive(zipToOpen, ZipArchiveMode.Update))
                {
                    ZipArchiveEntry projEntry = archive.CreateEntry("project.json");
                    using (StreamWriter writer = new StreamWriter(projEntry.Open()))
                    {
                        writer.Write(pluginState);
                    }
                }
            }

            using (FileStream zipToOpen = new FileStream(path, FileMode.Open))
            {
                using (ZipArchive archive = new ZipArchive(zipToOpen, ZipArchiveMode.Update))
                {
                    ZipArchiveEntry projEntry = archive.CreateEntry("image.png");
                    using (StreamWriter writer = new StreamWriter(projEntry.Open()))
                    {
                        Images.Last().Save(writer.BaseStream, ImageFormat.Png);
                    }
                }

                zipToOpen.Close();
                zipToOpen.Dispose();
            }
        }

        private void LoadProject(string path)
        {
            DestroyWorkspace();

            using (ZipArchive zip = ZipFile.Open(path, ZipArchiveMode.Read))
            {
                var projFile = zip.GetEntry("project.json");

                using (StreamReader s = new StreamReader(projFile.Open()))
                {
                    
                    List<InsertedPlugin> restoredPlugins = JsonConvert.DeserializeObject<List<InsertedPlugin>>(s.ReadToEnd());
                    foreach (var item in restoredPlugins)
                    {
                        Rack.AddPlugin(item);
                    }
                }

                var imgFile = zip.GetEntry("image.png");

                using (StreamReader s = new StreamReader(imgFile.Open()))
                {
                    Images.Add((Bitmap)Image.FromStream(s.BaseStream));
                }

                zip.Dispose();
            }

            SetPluginControls();
            SetImageControls();
            FillPluginList();
        }

        private void SetImageControls()
        {
            Text = "VSTImage " + SavePath ?? "untitled project";
            Log.Verbose("Image count: {0}", Images.Count);
            if (Images.Any() && Images.Last() != null)
            {
                toolApplyBtn.Enabled = true;
                exportToolStripMenuItem.Enabled = true;
                saveToolStripMenuItem.Enabled = true;
                pictureBox.Image = Images.Last();
            }
            else
            {
                toolApplyBtn.Enabled = false;
                exportToolStripMenuItem.Enabled = false;
                saveToolStripMenuItem.Enabled = false;
                pictureBox.Image = null;
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

                var selection = Rack.Plugins[listPlugins.SelectedItems[0].Index];

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

        private void DestroyImages()
        {
            foreach (var item in Images)
            {
                item.Dispose();
            }
            Images.Clear();
            SetImageControls();
        }

        private void FillPluginList()
        {
            listPlugins.Items.Clear();

            foreach (var ctx in Rack.Plugins)
            {
                ListViewItem lvItem = new ListViewItem(ctx.PluginContext.PluginCommandStub.Commands.GetEffectName());
                lvItem.SubItems.Add(ctx.PluginContext.PluginCommandStub.Commands.GetProductString());
                lvItem.SubItems.Add(ctx.PluginContext.PluginCommandStub.Commands.GetVendorString());
                lvItem.SubItems.Add(ctx.PluginContext.PluginCommandStub.Commands.GetVendorVersion().ToString());
                lvItem.SubItems.Add(ctx.PluginContext.Find<string>("PluginPath"));
                lvItem.Tag = ctx;

                listPlugins.Items.Add(lvItem);
            }

            openPluginEditorBtn.Enabled = Rack.Plugins.Any();
        }

        private void DestroyWorkspace()
        {
            Log.Verbose("Closing project..");
            listPlugins.Items.Clear();
            Rack.ReleaseAllPlugins();
            SetPluginControls();
            DestroyImages();
            SetImageControls();
            GC.Collect();
        }

        private void toolAddVstBtn_Click(object sender, EventArgs e)
        {
            openFileDlg.Filter = $"VST {Utils.GetArch()} 2.4 Plugins(*.dll)|*.dll";

            if (openFileDlg.ShowDialog(this) == DialogResult.OK)
            {
                var plugin = new InsertedPlugin(openFileDlg.FileName);
                Rack.AddPlugin(plugin);
                FillPluginList();
                SetPluginControls();
            }
        }

        private void openPluginEditorBtn_Click(object sender, EventArgs e)
        {
            Rack.Plugins[listPlugins.SelectedItems[0].Index].ShowEditor(this);
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

                    if (img.PixelFormat is PixelFormat.Indexed or PixelFormat.Format1bppIndexed or PixelFormat.Format4bppIndexed or PixelFormat.Format8bppIndexed)
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

        private void listPlugins_SelectedIndexChanged(object sender, EventArgs e)
        {
            SetPluginControls();
        }

        private void removePlugBtn_Click(object sender, EventArgs e)
        {
            var selected = listPlugins.SelectedItems[0].Index;
            var res = MessageBox.Show(this, "Remove plugin?", Rack.Plugins[selected].PluginContext.PluginCommandStub.Commands.GetEffectName(), MessageBoxButtons.YesNo, MessageBoxIcon.Warning);
            if (res == DialogResult.Yes)
            {
                if (Rack.Plugins[selected].PluginContext != null) { Rack.Plugins[selected].PluginContext.Dispose(); }
                Rack.Plugins.RemoveAt(selected);
                FillPluginList();
                SetPluginControls();
            }
        }

        private void hBox_SelectedIndexChanged(object sender, EventArgs e)
        {
            var selected = listPlugins.SelectedItems[0].Index;
            Rack.Plugins[selected].ProcessingValues[Channel.Hue] = (Processing)hBox.SelectedIndex;
        }

        private void sBox_SelectedIndexChanged(object sender, EventArgs e)
        {
            var selected = listPlugins.SelectedItems[0].Index;
            Rack.Plugins[selected].ProcessingValues[Channel.Saturation] = (Processing)sBox.SelectedIndex;
        }

        private void vBox_SelectedIndexChanged(object sender, EventArgs e)
        {
            var selected = listPlugins.SelectedItems[0].Index;
            Rack.Plugins[selected].ProcessingValues[Channel.Value] = (Processing)vBox.SelectedIndex;
        }

        private void trackWet_ValueChanged(object sender, EventArgs e)
        {
            var selected = listPlugins.SelectedItems[0].Index;
            Rack.Plugins[selected].Wet = trackWet.Value / 100; 
        }

        private void inputBox_SelectedIndexChanged(object sender, EventArgs e)
        {
            var selected = listPlugins.SelectedItems[0].Index;
            Rack.Plugins[selected].Input = (Channel)vBox.SelectedIndex;
        }

        private void backgroundWorker_DoWork(object sender, DoWorkEventArgs e)
        {
            var complete = 0;

            foreach (var plugin in Rack.Plugins)
            {
                Log.Information("Proccessing image...");
                ImageProcessor processor = new ImageProcessor(plugin);
                var image = processor.ProcessImage(Images.Last());
                Images.Add(image);
                complete++;
                backgroundWorker.ReportProgress(complete);
            }
        }

        private void backgroundWorker_ProgressChanged(object sender, ProgressChangedEventArgs e)
        {
            ProcessingProgress.UpdateProgress($"Effects processed: {e.ProgressPercentage} of {Rack.Plugins.Count}");
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
                    MessageBox.Show($"Saving error: {error.Message}", "Image saving failed!", MessageBoxButtons.OK, MessageBoxIcon.Error);
                }
            }
        }

        private void newProjectBtn_Click(object sender, EventArgs e)
        {
            SaveConformation();
            DestroyWorkspace();
            SavePath = null;
        }

        private void saveAsProjBtn_Click(object sender, EventArgs e)
        {
            SavePath = null;
            SaveProjUI();
        }

        private void loadProjBtn_Click(object sender, EventArgs e)
        {
            openFileDlg.Filter = "VSTImage project file(*.viproj)|*.viproj";

            if (openFileDlg.ShowDialog(this) == DialogResult.OK)
            {
                try
                {
                    SavePath = openFileDlg.FileName;
                    LoadProject(SavePath);
                }
                catch (Exception error)
                {
                    Log.Error("{0}", error);
                    MessageBox.Show($"Project loaded with errors: {error.Message}", "Project loading", MessageBoxButtons.OK, MessageBoxIcon.Error);
                }
            }
        }

        private void toolRemoveAllFX_Click(object sender, EventArgs e)
        {
            listPlugins.Items.Clear();
            Rack.ReleaseAllPlugins();
        }

        private void saveToolStripMenuItem_Click(object sender, EventArgs e)
        {
            SaveProjUI();
        }

        private void MainForm_FormClosing(object sender, FormClosingEventArgs e)
        {
            e.Cancel = SaveConformation();
            if (!e.Cancel)
            {
                Rack.ReleaseAllPlugins();
                foreach (var item in Images)
                {
                    item.Dispose();
                }
                Images.Clear();
            }
        }
    }
}
