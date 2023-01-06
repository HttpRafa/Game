using Scenes.Start.Scripts.Network;

namespace Scenes.Start.Scripts.Game.Network
{
    public class ChangeToNetworkStateInfo : StateChangeInfo
    {
        
        public string Address { get; private set; }
        public ushort Port { get; private set; }
        
        public NetworkMode NetworkMode { get; private set; }

        public ChangeToNetworkStateInfo(string address, ushort port, NetworkMode networkMode)
        {
            Address = address;
            Port = port;
            NetworkMode = networkMode;
        }
        
    }
}