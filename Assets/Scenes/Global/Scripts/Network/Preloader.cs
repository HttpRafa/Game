using System;
using System.Collections.Generic;
using Scenes.Global.Scripts.Logging;
using Unity.Netcode;
using Unity.Netcode.Transports.UTP;
using UnityEngine;

namespace Scenes.Global.Scripts.Network
{
    public class Preloader : MonoBehaviour
    {

        [SerializeField] private NetworkManager networkManager;
        [SerializeField] private UnityTransport networkTransport;

        [SerializeField] private bool autoHost;
        [SerializeField] private bool autoClient;
        
        private void Start()
        {
            if (Application.isEditor)
            {
                if (autoHost)
                {
                    networkManager.StartHost();
                    return;
                }

                if (autoClient)
                {
                    networkManager.StartClient();
                    return;
                }   
            }

            string[] arguments = Environment.GetCommandLineArgs();
            Dictionary<ArgumentData, object> argumentData = new Dictionary<ArgumentData, object>();
            
            GameLogger.Info("=> Start Argument Parsing");
            for (var i = 0; i < arguments.Length; i++)
            {
                switch (arguments[i])
                {
                    case "-address":
                        if ((i + 2) <= arguments.Length)
                        {
                            argumentData.Add(ArgumentData.Address, arguments[i + 1]);
                        }
                        else
                        {
                            GameLogger.Error("-address needs an value | Example: -address 127.0.0.1");
                        }
                        break;
                    case "-port":
                        if ((i + 2) <= arguments.Length)
                        {
                            if (ushort.TryParse(arguments[i + 1], out var value))
                            {
                                argumentData.Add(ArgumentData.Port, value);
                            }
                            else
                            {
                                GameLogger.Error(arguments[i + 1] + " is not an ushort");
                            }
                        }
                        else
                        {
                            GameLogger.Error("-port needs an value | Example: -port 9644");
                        }
                        break;
                    case "-client":
                        argumentData.Add(ArgumentData.Client, null);
                        break;
                    case "-host":
                        argumentData.Add(ArgumentData.Host, null);
                        break;
                }
            }
            foreach (var (key, value) in argumentData)
            {
                GameLogger.Info(key + ": " + value);
            }

            if (argumentData.ContainsKey(ArgumentData.Address))
            {
                networkTransport.ConnectionData.Address = (string)argumentData[ArgumentData.Address];
            }
            if (argumentData.ContainsKey(ArgumentData.Port))
            {
                networkTransport.ConnectionData.Port = (ushort)argumentData[ArgumentData.Port];
            }

# if UNITY_SERVER
            if (!Application.isEditor)
            {
                GameLogger.Info("=> GameServer | Start");
                GameLogger.Info("Starting server[" + networkTransport.ConnectionData.Address + ":" + networkTransport.ConnectionData.Port + "]");
                Application.targetFrameRate = 60;
                networkManager.StartServer();
            }
#else
            if (!Application.isEditor)
            {
                if (argumentData.ContainsKey(ArgumentData.Client))
                {
                    GameLogger.Info("=> Client | Start");
                    GameLogger.Info("Starting client[" + networkTransport.ConnectionData.Address + ":" +
                                    networkTransport.ConnectionData.Port + "]");
                    networkManager.StartClient();
                }
                if (argumentData.ContainsKey(ArgumentData.Host))
                {
                    GameLogger.Info("=> Host | Start");
                    GameLogger.Info("Starting host[" + networkTransport.ConnectionData.Address + ":" +
                                    networkTransport.ConnectionData.Port + "]");
                    networkManager.StartHost();
                }
            }
#endif
            
        }

        enum ArgumentData
        {
            Address,
            Port,
            
            Client,
            Host
        }
        
    }
}