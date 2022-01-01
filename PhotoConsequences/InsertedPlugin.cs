using Jacobi.Vst.Core;
using Jacobi.Vst.Host.Interop;
using Newtonsoft.Json;
using Serilog;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace PhotoConsequences
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

    public class InsertedPlugin
    {
        /// <summary>
        /// VST Plugin context
        /// </summary>
        [JsonIgnore]
        public VstPluginContext PluginContext { get; set; }
        /// <summary>
        /// Processing output channel pairs
        /// </summary>
        public Dictionary<Channel, Processing> ProcessingValues { get; set; }
        /// <summary>
        /// Processing input channel
        /// </summary>
        public Channel Input { get; set; }
        /// <summary>
        /// Dry/Wet processing ratio
        /// </summary>
        public float Wet { get; set; }
        /// <summary>
        /// Full path to VST
        /// </summary>
        public string PluginPath { get; set; }
        /// <summary>
        /// Plugin processing sample rate
        /// </summary>
        public float SampleRate { get; set; }
        /// <summary>
        /// Base64-encoded persistant plugin state. Use SetState() function to set this value
        /// </summary>
        public string PluginData { get; set; }

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
        /// Initializes a plugin chain
        /// </summary>
        /// <param name="pluginPath">VST 2.4 Plugin path</param>
        public InsertedPlugin(string pluginPath)
        {
            PluginPath = pluginPath;
            Wet = 1.0f;
            Input = Channel.Value;
            SampleRate = 44100;
            PluginData = string.Empty;
            ProcessingValues = new Dictionary<Channel, Processing>();
            ProcessingValues.Add(Channel.Hue, Processing.Left);
            ProcessingValues.Add(Channel.Saturation, Processing.None);
            ProcessingValues.Add(Channel.Value, Processing.None);
        }

        /// <summary>
        /// Initializes a plugin context
        /// </summary>
        public void CreatePluginContext()
        {
            HostCommandStub hostCmdStub = new HostCommandStub();
            hostCmdStub.PluginCalled += new EventHandler<PluginCalledEventArgs>(HostCmdStub_PluginCalled);

            PluginContext = VstPluginContext.Create(PluginPath, hostCmdStub);

            // add custom data to the context
            PluginContext.Set("PluginPath", PluginPath);
            PluginContext.Set("HostCmdStub", hostCmdStub);

            // actually open the plugin itself
            PluginContext.PluginCommandStub.Commands.Open();

            // plugin does not support processing audio
            if ((PluginContext.PluginInfo.Flags & VstPluginFlags.CanReplacing) == 0)
            {
                throw new InvalidOperationException("This plugin is not a effect");
            }

            if (PluginData != null && PluginData.Any())
            {
                Log.Information("Found base64 plugin parameters, LOADING NOW!");
                PluginContext.PluginCommandStub.Commands.SetChunk(Convert.FromBase64String(PluginData), true);
            }
        }

        /// <summary>
        /// Opens a plugin edior
        /// </summary>
        /// <param name="owner">Parent dialog window</param>
        public void ShowEditor(IWin32Window owner)
        {
            EditorFrame dlg = new EditorFrame
            {
                PluginCommandStub = PluginContext.PluginCommandStub
            };

            PluginContext.PluginCommandStub.Commands.MainsChanged(true);
            dlg.ShowDialog(owner);
            PluginContext.PluginCommandStub.Commands.MainsChanged(false);
        }

        /// <summary>
        /// Encodes plugin state to base64
        /// </summary>
        public void SetState()
        {
            Log.Verbose("Setting plugin state...");
            PluginData = Convert.ToBase64String(PluginContext.PluginCommandStub.Commands.GetChunk(true));
        }
    }
}
