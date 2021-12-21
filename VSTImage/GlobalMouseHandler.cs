using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace VSTImage
{

    public delegate void MouseMovedEvent();

    public class GlobalMouseHandler : IMessageFilter
    {
        private const int WM_MOUSEMOVE = 0x0200;

        public event MouseMovedEvent TheMouseMoved;

        #region IMessageFilter Members

        public bool PreFilterMessage(ref Message m)
        {
            if (m.Msg == WM_MOUSEMOVE)
            {
                TheMouseMoved?.Invoke();
            }
            // Always allow message to continue to the next filter control
            return false;
        }

        #endregion
    }
}
