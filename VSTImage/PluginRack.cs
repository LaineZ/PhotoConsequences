using Serilog;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace VSTImage
{
    public class PluginRack
    {
        public List<InsertedPlugin> Plugins = new List<InsertedPlugin>();


        public void ReleaseAllPlugins()
        {
            foreach (var ctx in Plugins) { ctx.PluginContext.Dispose();  }
            Plugins.Clear();
        }

        public void AddPlugin(InsertedPlugin plugin)
        {
            try
            {
                plugin.CreatePluginContext();

                if (plugin.PluginContext != null)
                {
                    Plugins.Add(plugin);
                }
                else
                {
                    MessageBox.Show("Failed to create plugin context", plugin.PluginPath, MessageBoxButtons.OK, MessageBoxIcon.Error);
                }
            }
            catch (BadImageFormatException ex)
            {
                Log.Error("Failed to open VST: {0}", ex.ToString());
                MessageBox.Show($"This VSTImage build can open only {Utils.GetArch()} plugin.", "Plugin load error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
            catch (EntryPointNotFoundException ex)
            {
                Log.Error("Failed to open VST: {0}", ex.ToString());
                MessageBox.Show($"This dll file is not a VST Plugin.", "Plugin load error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
            catch (Exception ex)
            {
                Log.Error("Failed to open VST: {0}", ex.ToString());
                MessageBox.Show(ex.ToString(), "Plugin load error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

    }
}
