using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Threading.Tasks;

namespace PhotoConsequences
{
    public static class Utils
    {
        public static ProcessorArchitecture GetArch()
        {
            Assembly currentAssem = Assembly.GetExecutingAssembly();
            return currentAssem.GetName().ProcessorArchitecture;
        }

        public static Version GetVersion()
        {
            Assembly currentAssem = Assembly.GetExecutingAssembly();
            return currentAssem.GetName().Version;
        }
    }
}
