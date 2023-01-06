using System.Collections.Generic;
using System.Linq;
using Scenes.Start.Scripts.Game.Network;
using Scenes.Start.Scripts.Network;
using Unity.Netcode;
using Unity.Netcode.Transports.UTP;
using UnityEngine;

namespace Scenes.Start.Scripts.Game
{
    
    public class GameStateManager : MonoBehaviour
    {

        [SerializeField] private List<GameState> gameStates = new();

        private GameState _gameState;

        private void Awake()
        {
            DontDestroyOnLoad(gameObject);
        }

        private void Start()
        {
            if (_gameState == null)
            {
                if (Application.platform == RuntimePlatform.LinuxServer ||
                    Application.platform == RuntimePlatform.WindowsServer ||
                    Application.platform == RuntimePlatform.OSXServer)
                {
                    ChangeState(FindGameState(GameState.GameStateType.Gameplay), new ChangeToNetworkStateInfo("192.168.0.200", 9644, NetworkMode.Server));
                }
                else
                {
                    ChangeState(FindGameState(GameState.GameStateType.Menu), null);
                }
            }
        }

        public void StartNetwork(NetworkMode networkMode)
        {
            if (networkMode == NetworkMode.Host)
            {
                NetworkManager.Singleton.StartHost();
            } else if (networkMode == NetworkMode.Client)
            {
                NetworkManager.Singleton.StartClient();
            } else if (networkMode == NetworkMode.Server)
            {
                NetworkManager.Singleton.StartServer();
            }
        }
        
        public GameState FindGameState(GameState.GameStateType type)
        {
            return gameStates.First(state => state.Type == type);
        }
        
        public async void ChangeState(GameState gameState, StateChangeInfo info)
        {
            if (_gameState != null)
            {
                if (_gameState.Type == gameState.Type) return;
                await _gameState.UnLoad();
            }

            await gameState.Load();
            
            if (gameState.Type == GameState.GameStateType.Gameplay && info != null && info is ChangeToNetworkStateInfo changeInfo)
            {
                UnityTransport unityTransport = NetworkManager.Singleton.NetworkConfig.NetworkTransport as UnityTransport;
                if (unityTransport != null)
                {
                    unityTransport.ConnectionData.Address = changeInfo.Address;
                    unityTransport.ConnectionData.Port = changeInfo.Port;   
                }
                
                StartNetwork(changeInfo.NetworkMode);
            }
            
        }
        
    }
    
}