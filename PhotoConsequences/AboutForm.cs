using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace PhotoConsequences
{
    public partial class AboutForm : Form
    {
        public AboutForm()
        {
            InitializeComponent();
        }

        private void OpenUrl(string url)
        {
            try
            {
                Process.Start(url);
            }
            catch
            {
                url = url.Replace("&", "^&");
                Process.Start(new ProcessStartInfo("cmd", $"/c start {url}") { CreateNoWindow = true });
            }
        }

        private void linkGit_Click(object sender, EventArgs e)
        {
            OpenUrl("https://github.com/LaineZ/PhotoConsequences");
        }

        private void AboutForm_Load(object sender, EventArgs e)
        {
            labelVersion.Text = "Version " + Utils.GetVersion();
        }
    }
}
