using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace PhotoConsequences
{
    public partial class ProcessingProgress : Form
    {
        public ProcessingProgress()
        {
            InitializeComponent();
        }

        public void UpdateProgress(string text)
        {
            labelProgress.Text = text;
        }
    }
}
