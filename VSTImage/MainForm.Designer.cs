
namespace VSTImage
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
            this.listPlugins = new System.Windows.Forms.ListView();
            this.NameHdr = new System.Windows.Forms.ColumnHeader();
            this.VendorHdr = new System.Windows.Forms.ColumnHeader();
            this.openPluginEditorBtn = new System.Windows.Forms.Button();
            this.groupBox1 = new System.Windows.Forms.GroupBox();
            this.groupBoxOptions = new System.Windows.Forms.GroupBox();
            this.inputBox = new System.Windows.Forms.ComboBox();
            this.label5 = new System.Windows.Forms.Label();
            this.undoBtn = new System.Windows.Forms.Button();
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
            this.toolOpenImageBtn = new System.Windows.Forms.ToolStripButton();
            this.toolSaveimgBtn = new System.Windows.Forms.ToolStripButton();
            this.toolAddVstBtn = new System.Windows.Forms.ToolStripButton();
            this.toolApplyBtn = new System.Windows.Forms.ToolStripButton();
            this.progressProcessing = new System.Windows.Forms.ToolStripProgressBar();
            this.openFileDlg = new System.Windows.Forms.OpenFileDialog();
            this.saveFileDlg = new System.Windows.Forms.SaveFileDialog();
            this.backgroundWorker = new System.ComponentModel.BackgroundWorker();
            this.pictureBox = new System.Windows.Forms.PictureBox();
            this.panelImage = new System.Windows.Forms.Panel();
            this.groupBox1.SuspendLayout();
            this.groupBoxOptions.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.trackWet)).BeginInit();
            this.toolStrip1.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.pictureBox)).BeginInit();
            this.panelImage.SuspendLayout();
            this.SuspendLayout();
            // 
            // listPlugins
            // 
            this.listPlugins.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom) 
            | System.Windows.Forms.AnchorStyles.Left)));
            this.listPlugins.Columns.AddRange(new System.Windows.Forms.ColumnHeader[] {
            this.NameHdr,
            this.VendorHdr});
            this.listPlugins.HideSelection = false;
            this.listPlugins.Location = new System.Drawing.Point(6, 17);
            this.listPlugins.Name = "listPlugins";
            this.listPlugins.Size = new System.Drawing.Size(313, 319);
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
            this.groupBox1.Location = new System.Drawing.Point(12, 28);
            this.groupBox1.Name = "groupBox1";
            this.groupBox1.Size = new System.Drawing.Size(337, 523);
            this.groupBox1.TabIndex = 3;
            this.groupBox1.TabStop = false;
            this.groupBox1.Text = "Plugins";
            // 
            // groupBoxOptions
            // 
            this.groupBoxOptions.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left)));
            this.groupBoxOptions.Controls.Add(this.inputBox);
            this.groupBoxOptions.Controls.Add(this.label5);
            this.groupBoxOptions.Controls.Add(this.undoBtn);
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
            this.groupBoxOptions.Location = new System.Drawing.Point(6, 342);
            this.groupBoxOptions.Name = "groupBoxOptions";
            this.groupBoxOptions.Size = new System.Drawing.Size(313, 173);
            this.groupBoxOptions.TabIndex = 3;
            this.groupBoxOptions.TabStop = false;
            this.groupBoxOptions.Text = "Plugin processing options";
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
            this.inputBox.Size = new System.Drawing.Size(177, 22);
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
            // undoBtn
            // 
            this.undoBtn.Location = new System.Drawing.Point(134, 145);
            this.undoBtn.Name = "undoBtn";
            this.undoBtn.Size = new System.Drawing.Size(74, 22);
            this.undoBtn.TabIndex = 13;
            this.undoBtn.Text = "Undo";
            this.undoBtn.UseVisualStyleBackColor = true;
            this.undoBtn.Click += new System.EventHandler(this.undoBtn_Click);
            // 
            // removePlugBtn
            // 
            this.removePlugBtn.Enabled = false;
            this.removePlugBtn.Location = new System.Drawing.Point(214, 145);
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
            this.trackWet.Size = new System.Drawing.Size(256, 45);
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
            this.sBox.Location = new System.Drawing.Point(134, 46);
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
            this.vBox.Location = new System.Drawing.Point(231, 46);
            this.vBox.Name = "vBox";
            this.vBox.Size = new System.Drawing.Size(64, 22);
            this.vBox.TabIndex = 8;
            this.vBox.SelectedIndexChanged += new System.EventHandler(this.vBox_SelectedIndexChanged);
            // 
            // label3
            // 
            this.label3.AutoSize = true;
            this.label3.Location = new System.Drawing.Point(204, 49);
            this.label3.Name = "label3";
            this.label3.Size = new System.Drawing.Size(21, 14);
            this.label3.TabIndex = 7;
            this.label3.Text = "V:";
            // 
            // label2
            // 
            this.label2.AutoSize = true;
            this.label2.Location = new System.Drawing.Point(107, 49);
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
            this.hBox.Location = new System.Drawing.Point(37, 46);
            this.hBox.Name = "hBox";
            this.hBox.Size = new System.Drawing.Size(64, 22);
            this.hBox.TabIndex = 4;
            this.hBox.SelectedIndexChanged += new System.EventHandler(this.hBox_SelectedIndexChanged);
            // 
            // label1
            // 
            this.label1.AutoSize = true;
            this.label1.Location = new System.Drawing.Point(10, 49);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(21, 14);
            this.label1.TabIndex = 3;
            this.label1.Text = "H:";
            // 
            // toolStrip1
            // 
            this.toolStrip1.AllowItemReorder = true;
            this.toolStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.toolOpenImageBtn,
            this.toolSaveimgBtn,
            this.toolAddVstBtn,
            this.toolApplyBtn,
            this.progressProcessing});
            this.toolStrip1.Location = new System.Drawing.Point(0, 0);
            this.toolStrip1.Name = "toolStrip1";
            this.toolStrip1.Size = new System.Drawing.Size(920, 25);
            this.toolStrip1.TabIndex = 4;
            this.toolStrip1.Text = "toolStrip1";
            // 
            // toolOpenImageBtn
            // 
            this.toolOpenImageBtn.Image = global::VSTImage.Properties.Resources.computer_folder_open;
            this.toolOpenImageBtn.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.toolOpenImageBtn.Name = "toolOpenImageBtn";
            this.toolOpenImageBtn.Size = new System.Drawing.Size(97, 22);
            this.toolOpenImageBtn.Text = "Open image";
            this.toolOpenImageBtn.Click += new System.EventHandler(this.toolOpenImageBtn_Click);
            // 
            // toolSaveimgBtn
            // 
            this.toolSaveimgBtn.Enabled = false;
            this.toolSaveimgBtn.Image = global::VSTImage.Properties.Resources.card_memory_sd;
            this.toolSaveimgBtn.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.toolSaveimgBtn.Name = "toolSaveimgBtn";
            this.toolSaveimgBtn.Size = new System.Drawing.Size(97, 22);
            this.toolSaveimgBtn.Text = "Save image";
            this.toolSaveimgBtn.Click += new System.EventHandler(this.toolSaveimgBtn_Click);
            // 
            // toolAddVstBtn
            // 
            this.toolAddVstBtn.Image = global::VSTImage.Properties.Resources.plug;
            this.toolAddVstBtn.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.toolAddVstBtn.Name = "toolAddVstBtn";
            this.toolAddVstBtn.Size = new System.Drawing.Size(97, 22);
            this.toolAddVstBtn.Text = "Add VST FX";
            this.toolAddVstBtn.Click += new System.EventHandler(this.toolAddVstBtn_Click);
            // 
            // toolApplyBtn
            // 
            this.toolApplyBtn.Enabled = false;
            this.toolApplyBtn.Image = global::VSTImage.Properties.Resources.check_mark_square;
            this.toolApplyBtn.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.toolApplyBtn.Name = "toolApplyBtn";
            this.toolApplyBtn.Size = new System.Drawing.Size(146, 22);
            this.toolApplyBtn.Text = "Apply FX on image";
            this.toolApplyBtn.Click += new System.EventHandler(this.toolApplyBtn_Click);
            // 
            // progressProcessing
            // 
            this.progressProcessing.Name = "progressProcessing";
            this.progressProcessing.Size = new System.Drawing.Size(100, 22);
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
            this.pictureBox.ErrorImage = global::VSTImage.Properties.Resources.empty;
            this.pictureBox.InitialImage = global::VSTImage.Properties.Resources.empty;
            this.pictureBox.Location = new System.Drawing.Point(3, 3);
            this.pictureBox.Name = "pictureBox";
            this.pictureBox.Size = new System.Drawing.Size(553, 517);
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
            this.panelImage.Location = new System.Drawing.Point(355, 28);
            this.panelImage.Name = "panelImage";
            this.panelImage.Size = new System.Drawing.Size(565, 523);
            this.panelImage.TabIndex = 5;
            // 
            // MainForm
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(7F, 14F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(920, 555);
            this.Controls.Add(this.panelImage);
            this.Controls.Add(this.groupBox1);
            this.Controls.Add(this.toolStrip1);
            this.Name = "MainForm";
            this.Text = "VSTImage";
            this.FormClosed += new System.Windows.Forms.FormClosedEventHandler(this.Form1_FormClosed);
            this.Load += new System.EventHandler(this.Form1_Load);
            this.groupBox1.ResumeLayout(false);
            this.groupBoxOptions.ResumeLayout(false);
            this.groupBoxOptions.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.trackWet)).EndInit();
            this.toolStrip1.ResumeLayout(false);
            this.toolStrip1.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.pictureBox)).EndInit();
            this.panelImage.ResumeLayout(false);
            this.panelImage.PerformLayout();
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.ListView listPlugins;
        private System.Windows.Forms.Button openPluginEditorBtn;
        private System.Windows.Forms.GroupBox groupBox1;
        private System.Windows.Forms.ToolStrip toolStrip1;
        private System.Windows.Forms.ToolStripButton toolOpenImageBtn;
        private System.Windows.Forms.ToolStripButton toolSaveimgBtn;
        private System.Windows.Forms.ToolStripButton toolAddVstBtn;
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
        private System.Windows.Forms.ToolStripProgressBar progressProcessing;
        private System.Windows.Forms.Button undoBtn;
        private System.Windows.Forms.PictureBox pictureBox;
        private System.Windows.Forms.Panel panelImage;
        private System.Windows.Forms.ComboBox inputBox;
        private System.Windows.Forms.Label label5;
    }
}

