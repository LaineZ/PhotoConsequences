
namespace PhotoConsequences
{
    partial class MainForm
    {
        /// <summary>
        ///  Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        ///  Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        ///  Required method for Designer support - do not modify
        ///  the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(MainForm));
            this.listPlugins = new System.Windows.Forms.ListView();
            this.NameHdr = new System.Windows.Forms.ColumnHeader();
            this.VendorHdr = new System.Windows.Forms.ColumnHeader();
            this.ProductHdr = new System.Windows.Forms.ColumnHeader();
            this.openPluginEditorBtn = new System.Windows.Forms.Button();
            this.groupBox1 = new System.Windows.Forms.GroupBox();
            this.groupBoxOptions = new System.Windows.Forms.GroupBox();
            this.label6 = new System.Windows.Forms.Label();
            this.sampleRateInput = new System.Windows.Forms.NumericUpDown();
            this.inputBox = new System.Windows.Forms.ComboBox();
            this.label5 = new System.Windows.Forms.Label();
            this.removePlugBtn = new System.Windows.Forms.Button();
            this.label4 = new System.Windows.Forms.Label();
            this.trackWet = new System.Windows.Forms.TrackBar();
            this.sBox = new System.Windows.Forms.ComboBox();
            this.vBox = new System.Windows.Forms.ComboBox();
            this.label3 = new System.Windows.Forms.Label();
            this.label2 = new System.Windows.Forms.Label();
            this.hBox = new System.Windows.Forms.ComboBox();
            this.label1 = new System.Windows.Forms.Label();
            this.toolStrip1 = new System.Windows.Forms.ToolStrip();
            this.toolUndoBtn = new System.Windows.Forms.ToolStripButton();
            this.toolApplyBtn = new System.Windows.Forms.ToolStripButton();
            this.toolAddVstBtn = new System.Windows.Forms.ToolStripButton();
            this.toolRemoveAllFX = new System.Windows.Forms.ToolStripButton();
            this.toolOpenImageBtn = new System.Windows.Forms.ToolStripButton();
            this.openFileDlg = new System.Windows.Forms.OpenFileDialog();
            this.saveFileDlg = new System.Windows.Forms.SaveFileDialog();
            this.backgroundWorker = new System.ComponentModel.BackgroundWorker();
            this.pictureBox = new System.Windows.Forms.PictureBox();
            this.panelImage = new System.Windows.Forms.Panel();
            this.colorDialog1 = new System.Windows.Forms.ColorDialog();
            this.menuStrip = new System.Windows.Forms.MenuStrip();
            this.fileToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.newToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.openToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.saveToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.saveAsToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.exportToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.exitToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.aboutToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.groupBox1.SuspendLayout();
            this.groupBoxOptions.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.sampleRateInput)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.trackWet)).BeginInit();
            this.toolStrip1.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.pictureBox)).BeginInit();
            this.panelImage.SuspendLayout();
            this.menuStrip.SuspendLayout();
            this.SuspendLayout();
            // 
            // listPlugins
            // 
            this.listPlugins.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom) 
            | System.Windows.Forms.AnchorStyles.Left)));
            this.listPlugins.Columns.AddRange(new System.Windows.Forms.ColumnHeader[] {
            this.NameHdr,
            this.VendorHdr,
            this.ProductHdr});
            this.listPlugins.HideSelection = false;
            this.listPlugins.Location = new System.Drawing.Point(6, 22);
            this.listPlugins.Name = "listPlugins";
            this.listPlugins.Size = new System.Drawing.Size(325, 239);
            this.listPlugins.TabIndex = 0;
            this.listPlugins.TabStop = false;
            this.listPlugins.UseCompatibleStateImageBehavior = false;
            this.listPlugins.View = System.Windows.Forms.View.Details;
            this.listPlugins.SelectedIndexChanged += new System.EventHandler(this.listPlugins_SelectedIndexChanged);
            // 
            // NameHdr
            // 
            this.NameHdr.Text = "Name";
            // 
            // VendorHdr
            // 
            this.VendorHdr.Text = "Vendor";
            // 
            // ProductHdr
            // 
            this.ProductHdr.Text = "Product";
            // 
            // openPluginEditorBtn
            // 
            this.openPluginEditorBtn.Enabled = false;
            this.openPluginEditorBtn.Location = new System.Drawing.Point(10, 145);
            this.openPluginEditorBtn.Name = "openPluginEditorBtn";
            this.openPluginEditorBtn.Size = new System.Drawing.Size(118, 22);
            this.openPluginEditorBtn.TabIndex = 2;
            this.openPluginEditorBtn.Text = "Plugin editor";
            this.openPluginEditorBtn.UseVisualStyleBackColor = true;
            this.openPluginEditorBtn.Click += new System.EventHandler(this.openPluginEditorBtn_Click);
            // 
            // groupBox1
            // 
            this.groupBox1.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom) 
            | System.Windows.Forms.AnchorStyles.Left)));
            this.groupBox1.Controls.Add(this.groupBoxOptions);
            this.groupBox1.Controls.Add(this.listPlugins);
            this.groupBox1.Location = new System.Drawing.Point(12, 52);
            this.groupBox1.Name = "groupBox1";
            this.groupBox1.Size = new System.Drawing.Size(337, 448);
            this.groupBox1.TabIndex = 3;
            this.groupBox1.TabStop = false;
            this.groupBox1.Text = "Plugins";
            // 
            // groupBoxOptions
            // 
            this.groupBoxOptions.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left)));
            this.groupBoxOptions.Controls.Add(this.label6);
            this.groupBoxOptions.Controls.Add(this.sampleRateInput);
            this.groupBoxOptions.Controls.Add(this.inputBox);
            this.groupBoxOptions.Controls.Add(this.label5);
            this.groupBoxOptions.Controls.Add(this.removePlugBtn);
            this.groupBoxOptions.Controls.Add(this.label4);
            this.groupBoxOptions.Controls.Add(this.trackWet);
            this.groupBoxOptions.Controls.Add(this.openPluginEditorBtn);
            this.groupBoxOptions.Controls.Add(this.sBox);
            this.groupBoxOptions.Controls.Add(this.vBox);
            this.groupBoxOptions.Controls.Add(this.label3);
            this.groupBoxOptions.Controls.Add(this.label2);
            this.groupBoxOptions.Controls.Add(this.hBox);
            this.groupBoxOptions.Controls.Add(this.label1);
            this.groupBoxOptions.Location = new System.Drawing.Point(6, 267);
            this.groupBoxOptions.Name = "groupBoxOptions";
            this.groupBoxOptions.Size = new System.Drawing.Size(325, 173);
            this.groupBoxOptions.TabIndex = 3;
            this.groupBoxOptions.TabStop = false;
            this.groupBoxOptions.Text = "Plugin processing options";
            // 
            // label6
            // 
            this.label6.AutoSize = true;
            this.label6.Location = new System.Drawing.Point(10, 105);
            this.label6.Name = "label6";
            this.label6.Size = new System.Drawing.Size(91, 14);
            this.label6.TabIndex = 17;
            this.label6.Text = "Sample rate:";
            // 
            // sampleRateInput
            // 
            this.sampleRateInput.Location = new System.Drawing.Point(105, 103);
            this.sampleRateInput.Maximum = new decimal(new int[] {
            192000,
            0,
            0,
            0});
            this.sampleRateInput.Minimum = new decimal(new int[] {
            22005,
            0,
            0,
            0});
            this.sampleRateInput.Name = "sampleRateInput";
            this.sampleRateInput.Size = new System.Drawing.Size(120, 23);
            this.sampleRateInput.TabIndex = 16;
            this.sampleRateInput.Value = new decimal(new int[] {
            44100,
            0,
            0,
            0});
            // 
            // inputBox
            // 
            this.inputBox.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
            this.inputBox.Enabled = false;
            this.inputBox.FormattingEnabled = true;
            this.inputBox.Items.AddRange(new object[] {
            "Hue",
            "Saturation",
            "Value",
            "Random"});
            this.inputBox.Location = new System.Drawing.Point(118, 19);
            this.inputBox.Name = "inputBox";
            this.inputBox.Size = new System.Drawing.Size(201, 22);
            this.inputBox.TabIndex = 15;
            this.inputBox.SelectedIndexChanged += new System.EventHandler(this.inputBox_SelectedIndexChanged);
            // 
            // label5
            // 
            this.label5.AutoSize = true;
            this.label5.Location = new System.Drawing.Point(7, 22);
            this.label5.Name = "label5";
            this.label5.Size = new System.Drawing.Size(105, 14);
            this.label5.TabIndex = 14;
            this.label5.Text = "Input channel:";
            // 
            // removePlugBtn
            // 
            this.removePlugBtn.Enabled = false;
            this.removePlugBtn.Location = new System.Drawing.Point(132, 145);
            this.removePlugBtn.Name = "removePlugBtn";
            this.removePlugBtn.Size = new System.Drawing.Size(93, 22);
            this.removePlugBtn.TabIndex = 12;
            this.removePlugBtn.Text = "Remove";
            this.removePlugBtn.UseVisualStyleBackColor = true;
            this.removePlugBtn.Click += new System.EventHandler(this.removePlugBtn_Click);
            // 
            // label4
            // 
            this.label4.AutoSize = true;
            this.label4.Location = new System.Drawing.Point(10, 82);
            this.label4.Name = "label4";
            this.label4.Size = new System.Drawing.Size(35, 14);
            this.label4.TabIndex = 11;
            this.label4.Text = "Wet:";
            // 
            // trackWet
            // 
            this.trackWet.Enabled = false;
            this.trackWet.Location = new System.Drawing.Point(39, 74);
            this.trackWet.Maximum = 100;
            this.trackWet.Name = "trackWet";
            this.trackWet.Size = new System.Drawing.Size(280, 45);
            this.trackWet.TabIndex = 10;
            this.trackWet.Value = 100;
            this.trackWet.ValueChanged += new System.EventHandler(this.trackWet_ValueChanged);
            // 
            // sBox
            // 
            this.sBox.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
            this.sBox.Enabled = false;
            this.sBox.FormattingEnabled = true;
            this.sBox.Items.AddRange(new object[] {
            "Left",
            "Right",
            "None"});
            this.sBox.Location = new System.Drawing.Point(134, 47);
            this.sBox.Name = "sBox";
            this.sBox.Size = new System.Drawing.Size(64, 22);
            this.sBox.TabIndex = 9;
            this.sBox.SelectedIndexChanged += new System.EventHandler(this.sBox_SelectedIndexChanged);
            // 
            // vBox
            // 
            this.vBox.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
            this.vBox.Enabled = false;
            this.vBox.FormattingEnabled = true;
            this.vBox.Items.AddRange(new object[] {
            "Left",
            "Right",
            "None"});
            this.vBox.Location = new System.Drawing.Point(231, 47);
            this.vBox.Name = "vBox";
            this.vBox.Size = new System.Drawing.Size(64, 22);
            this.vBox.TabIndex = 8;
            this.vBox.SelectedIndexChanged += new System.EventHandler(this.vBox_SelectedIndexChanged);
            // 
            // label3
            // 
            this.label3.AutoSize = true;
            this.label3.Location = new System.Drawing.Point(204, 50);
            this.label3.Name = "label3";
            this.label3.Size = new System.Drawing.Size(21, 14);
            this.label3.TabIndex = 7;
            this.label3.Text = "V:";
            // 
            // label2
            // 
            this.label2.AutoSize = true;
            this.label2.Location = new System.Drawing.Point(107, 50);
            this.label2.Name = "label2";
            this.label2.Size = new System.Drawing.Size(21, 14);
            this.label2.TabIndex = 5;
            this.label2.Text = "S:";
            // 
            // hBox
            // 
            this.hBox.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
            this.hBox.Enabled = false;
            this.hBox.FormattingEnabled = true;
            this.hBox.Items.AddRange(new object[] {
            "Left",
            "Right",
            "None"});
            this.hBox.Location = new System.Drawing.Point(37, 47);
            this.hBox.Name = "hBox";
            this.hBox.Size = new System.Drawing.Size(64, 22);
            this.hBox.TabIndex = 4;
            this.hBox.SelectedIndexChanged += new System.EventHandler(this.hBox_SelectedIndexChanged);
            // 
            // label1
            // 
            this.label1.AutoSize = true;
            this.label1.Location = new System.Drawing.Point(10, 50);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(21, 14);
            this.label1.TabIndex = 3;
            this.label1.Text = "H:";
            // 
            // toolStrip1
            // 
            this.toolStrip1.AllowItemReorder = true;
            this.toolStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.toolUndoBtn,
            this.toolApplyBtn,
            this.toolAddVstBtn,
            this.toolRemoveAllFX,
            this.toolOpenImageBtn});
            this.toolStrip1.Location = new System.Drawing.Point(0, 24);
            this.toolStrip1.Name = "toolStrip1";
            this.toolStrip1.Size = new System.Drawing.Size(944, 25);
            this.toolStrip1.TabIndex = 4;
            this.toolStrip1.Text = "toolStrip1";
            // 
            // toolUndoBtn
            // 
            this.toolUndoBtn.DisplayStyle = System.Windows.Forms.ToolStripItemDisplayStyle.Text;
            this.toolUndoBtn.Image = ((System.Drawing.Image)(resources.GetObject("toolUndoBtn.Image")));
            this.toolUndoBtn.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.toolUndoBtn.Name = "toolUndoBtn";
            this.toolUndoBtn.Size = new System.Drawing.Size(39, 22);
            this.toolUndoBtn.Text = "Undo";
            this.toolUndoBtn.Click += new System.EventHandler(this.undoBtn_Click);
            // 
            // toolApplyBtn
            // 
            this.toolApplyBtn.Enabled = false;
            this.toolApplyBtn.Image = global::PhotoConsequences.Properties.Resources.check_mark_square;
            this.toolApplyBtn.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.toolApplyBtn.Name = "toolApplyBtn";
            this.toolApplyBtn.Size = new System.Drawing.Size(146, 22);
            this.toolApplyBtn.Text = "Apply FX on image";
            this.toolApplyBtn.Click += new System.EventHandler(this.toolApplyBtn_Click);
            // 
            // toolAddVstBtn
            // 
            this.toolAddVstBtn.Image = global::PhotoConsequences.Properties.Resources.ic_add_18pt;
            this.toolAddVstBtn.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.toolAddVstBtn.Name = "toolAddVstBtn";
            this.toolAddVstBtn.Size = new System.Drawing.Size(97, 22);
            this.toolAddVstBtn.Text = "Add VST Fx";
            this.toolAddVstBtn.Click += new System.EventHandler(this.toolAddVstBtn_Click);
            // 
            // toolRemoveAllFX
            // 
            this.toolRemoveAllFX.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.toolRemoveAllFX.Name = "toolRemoveAllFX";
            this.toolRemoveAllFX.Size = new System.Drawing.Size(102, 22);
            this.toolRemoveAllFX.Text = "Remove all FX";
            this.toolRemoveAllFX.Click += new System.EventHandler(this.toolRemoveAllFX_Click);
            // 
            // toolOpenImageBtn
            // 
            this.toolOpenImageBtn.Image = global::PhotoConsequences.Properties.Resources.ic_open_in_browser_18pt;
            this.toolOpenImageBtn.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.toolOpenImageBtn.Name = "toolOpenImageBtn";
            this.toolOpenImageBtn.Size = new System.Drawing.Size(97, 22);
            this.toolOpenImageBtn.Text = "Open image";
            this.toolOpenImageBtn.Click += new System.EventHandler(this.toolOpenImageBtn_Click);
            // 
            // openFileDlg
            // 
            this.openFileDlg.FileName = "openFileDialog1";
            // 
            // backgroundWorker
            // 
            this.backgroundWorker.WorkerReportsProgress = true;
            this.backgroundWorker.DoWork += new System.ComponentModel.DoWorkEventHandler(this.backgroundWorker_DoWork);
            this.backgroundWorker.ProgressChanged += new System.ComponentModel.ProgressChangedEventHandler(this.backgroundWorker_ProgressChanged);
            this.backgroundWorker.RunWorkerCompleted += new System.ComponentModel.RunWorkerCompletedEventHandler(this.backgroundWorker_RunWorkerCompleted);
            // 
            // pictureBox
            // 
            this.pictureBox.BackgroundImageLayout = System.Windows.Forms.ImageLayout.Zoom;
            this.pictureBox.Location = new System.Drawing.Point(3, 3);
            this.pictureBox.Name = "pictureBox";
            this.pictureBox.Size = new System.Drawing.Size(32, 32);
            this.pictureBox.SizeMode = System.Windows.Forms.PictureBoxSizeMode.AutoSize;
            this.pictureBox.TabIndex = 0;
            this.pictureBox.TabStop = false;
            // 
            // panelImage
            // 
            this.panelImage.Anchor = ((System.Windows.Forms.AnchorStyles)((((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom) 
            | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.panelImage.AutoScroll = true;
            this.panelImage.Controls.Add(this.pictureBox);
            this.panelImage.Location = new System.Drawing.Point(355, 52);
            this.panelImage.Name = "panelImage";
            this.panelImage.Size = new System.Drawing.Size(589, 448);
            this.panelImage.TabIndex = 5;
            // 
            // menuStrip
            // 
            this.menuStrip.Items.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.fileToolStripMenuItem,
            this.aboutToolStripMenuItem});
            this.menuStrip.Location = new System.Drawing.Point(0, 0);
            this.menuStrip.Name = "menuStrip";
            this.menuStrip.Size = new System.Drawing.Size(944, 24);
            this.menuStrip.TabIndex = 6;
            this.menuStrip.Text = "menuStrip1";
            // 
            // fileToolStripMenuItem
            // 
            this.fileToolStripMenuItem.DropDownItems.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.newToolStripMenuItem,
            this.openToolStripMenuItem,
            this.saveToolStripMenuItem,
            this.saveAsToolStripMenuItem,
            this.exportToolStripMenuItem,
            this.exitToolStripMenuItem});
            this.fileToolStripMenuItem.Name = "fileToolStripMenuItem";
            this.fileToolStripMenuItem.Size = new System.Drawing.Size(47, 20);
            this.fileToolStripMenuItem.Text = "File";
            // 
            // newToolStripMenuItem
            // 
            this.newToolStripMenuItem.Image = global::PhotoConsequences.Properties.Resources.empty;
            this.newToolStripMenuItem.Name = "newToolStripMenuItem";
            this.newToolStripMenuItem.Size = new System.Drawing.Size(180, 22);
            this.newToolStripMenuItem.Text = "New";
            this.newToolStripMenuItem.Click += new System.EventHandler(this.newProjectBtn_Click);
            // 
            // openToolStripMenuItem
            // 
            this.openToolStripMenuItem.Image = global::PhotoConsequences.Properties.Resources.computer_folder_open;
            this.openToolStripMenuItem.Name = "openToolStripMenuItem";
            this.openToolStripMenuItem.Size = new System.Drawing.Size(180, 22);
            this.openToolStripMenuItem.Text = "Open";
            this.openToolStripMenuItem.Click += new System.EventHandler(this.loadProjBtn_Click);
            // 
            // saveToolStripMenuItem
            // 
            this.saveToolStripMenuItem.Enabled = false;
            this.saveToolStripMenuItem.Image = global::PhotoConsequences.Properties.Resources.card_memory_sd;
            this.saveToolStripMenuItem.Name = "saveToolStripMenuItem";
            this.saveToolStripMenuItem.Size = new System.Drawing.Size(180, 22);
            this.saveToolStripMenuItem.Text = "Save";
            this.saveToolStripMenuItem.Click += new System.EventHandler(this.saveToolStripMenuItem_Click);
            // 
            // saveAsToolStripMenuItem
            // 
            this.saveAsToolStripMenuItem.Enabled = false;
            this.saveAsToolStripMenuItem.Name = "saveAsToolStripMenuItem";
            this.saveAsToolStripMenuItem.Size = new System.Drawing.Size(180, 22);
            this.saveAsToolStripMenuItem.Text = "Save as...";
            this.saveAsToolStripMenuItem.Click += new System.EventHandler(this.saveAsProjBtn_Click);
            // 
            // exportToolStripMenuItem
            // 
            this.exportToolStripMenuItem.Enabled = false;
            this.exportToolStripMenuItem.Name = "exportToolStripMenuItem";
            this.exportToolStripMenuItem.Size = new System.Drawing.Size(180, 22);
            this.exportToolStripMenuItem.Text = "Export image";
            this.exportToolStripMenuItem.Click += new System.EventHandler(this.toolSaveimgBtn_Click);
            // 
            // exitToolStripMenuItem
            // 
            this.exitToolStripMenuItem.Name = "exitToolStripMenuItem";
            this.exitToolStripMenuItem.Size = new System.Drawing.Size(180, 22);
            this.exitToolStripMenuItem.Text = "Exit";
            // 
            // aboutToolStripMenuItem
            // 
            this.aboutToolStripMenuItem.Name = "aboutToolStripMenuItem";
            this.aboutToolStripMenuItem.Size = new System.Drawing.Size(54, 20);
            this.aboutToolStripMenuItem.Text = "About";
            this.aboutToolStripMenuItem.Click += new System.EventHandler(this.aboutToolStripMenuItem_Click);
            // 
            // MainForm
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(7F, 14F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(944, 504);
            this.Controls.Add(this.panelImage);
            this.Controls.Add(this.groupBox1);
            this.Controls.Add(this.toolStrip1);
            this.Controls.Add(this.menuStrip);
            this.MainMenuStrip = this.menuStrip;
            this.Name = "MainForm";
            this.Text = "PhotoConsequences";
            this.FormClosing += new System.Windows.Forms.FormClosingEventHandler(this.MainForm_FormClosing);
            this.groupBox1.ResumeLayout(false);
            this.groupBoxOptions.ResumeLayout(false);
            this.groupBoxOptions.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.sampleRateInput)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.trackWet)).EndInit();
            this.toolStrip1.ResumeLayout(false);
            this.toolStrip1.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.pictureBox)).EndInit();
            this.panelImage.ResumeLayout(false);
            this.panelImage.PerformLayout();
            this.menuStrip.ResumeLayout(false);
            this.menuStrip.PerformLayout();
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.ListView listPlugins;
        private System.Windows.Forms.Button openPluginEditorBtn;
        private System.Windows.Forms.GroupBox groupBox1;
        private System.Windows.Forms.ToolStrip toolStrip1;
        private System.Windows.Forms.ToolStripButton toolApplyBtn;
        private System.Windows.Forms.OpenFileDialog openFileDlg;
        private System.Windows.Forms.SaveFileDialog saveFileDlg;
        private System.Windows.Forms.ColumnHeader NameHdr;
        private System.Windows.Forms.ColumnHeader VendorHdr;
        private System.Windows.Forms.GroupBox groupBoxOptions;
        private System.Windows.Forms.Label label1;
        private System.Windows.Forms.Label label3;
        private System.Windows.Forms.Label label2;
        private System.Windows.Forms.ComboBox hBox;
        private System.Windows.Forms.ComboBox sBox;
        private System.Windows.Forms.ComboBox vBox;
        private System.Windows.Forms.Label label4;
        private System.Windows.Forms.TrackBar trackWet;
        private System.Windows.Forms.Button removePlugBtn;
        private System.ComponentModel.BackgroundWorker backgroundWorker;
        private System.Windows.Forms.PictureBox pictureBox;
        private System.Windows.Forms.Panel panelImage;
        private System.Windows.Forms.ComboBox inputBox;
        private System.Windows.Forms.Label label5;
        private System.Windows.Forms.Label label6;
        private System.Windows.Forms.NumericUpDown sampleRateInput;
        private System.Windows.Forms.ColumnHeader ProductHdr;
        private System.Windows.Forms.ToolStripButton toolUndoBtn;
        private System.Windows.Forms.ToolStripButton toolAddVstBtn;
        private System.Windows.Forms.ToolStripButton toolRemoveAllFX;
        private System.Windows.Forms.ColorDialog colorDialog1;
        private System.Windows.Forms.MenuStrip menuStrip;
        private System.Windows.Forms.ToolStripMenuItem fileToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem newToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem openToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem saveToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem saveAsToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem exportToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem exitToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem aboutToolStripMenuItem;
        private System.Windows.Forms.ToolStripButton toolOpenImageBtn;
    }
}

