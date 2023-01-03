using Unity.Netcode.Components;
using UnityEngine;

namespace Scenes.Global.Scripts.Network
{
    
    [DisallowMultipleComponent]
    public class ClientNetworkTransform : NetworkTransform
    {
        protected override bool OnIsServerAuthoritative()
        {
            return false;
        }
    }
    
}