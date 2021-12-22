using Jacobi.Vst.Core;
using Jacobi.Vst.Host.Interop;
using Serilog;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace VSTImage
{
    public enum Processing
    {
        Left = 0,
        Right = 1,
        None = 2,
    }

    public enum Channel
    {
        Hue,
        Saturation,
        Value,
        Random,
    }

    class PluginChain
    {
        public VstPluginContext PluginContext { get; set; }
        public Dictionary<Channel, Processing> ProcessingValues { get; set; }
        public Channel Input { get; set; }
        public float Dry { get; set; }

        public PluginChain(VstPluginContext ctx)
        {
            PluginContext = ctx;
            Dry = 1.0f;
            Input = Channel.Value;
            ProcessingValues = new Dictionary<Channel, Processing>();
            ProcessingValues.Add(Channel.Hue, Processing.Left);
            ProcessingValues.Add(Channel.Saturation, Processing.None);
            ProcessingValues.Add(Channel.Value, Processing.None);
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

        /// <summary>
        /// Initializes a plugin context
        /// </summary>
        /// <param name="pluginPath">VST 2.4 Plugin path</param>
        public PluginChain(string pluginPath)
        {
            HostCommandStub hostCmdStub = new HostCommandStub();
            hostCmdStub.PluginCalled += new EventHandler<PluginCalledEventArgs>(HostCmdStub_PluginCalled);

            PluginContext = VstPluginContext.Create(pluginPath, hostCmdStub);

            // add custom data to the context
            PluginContext.Set("PluginPath", pluginPath);
            PluginContext.Set("HostCmdStub", hostCmdStub);

            // actually open the plugin itself
            PluginContext.PluginCommandStub.Commands.Open();

            // plugin does not support processing audio
            if ((PluginContext.PluginInfo.Flags & VstPluginFlags.CanReplacing) == 0)
            {
                throw new InvalidOperationException("This plugin is not a effect");
            }

            Dry = 1.0f;
            Input = Channel.Value;
            ProcessingValues = new Dictionary<Channel, Processing>();
            ProcessingValues.Add(Channel.Hue, Processing.Left);
            ProcessingValues.Add(Channel.Saturation, Processing.None);
            ProcessingValues.Add(Channel.Value, Processing.None);
        }
    }
}
