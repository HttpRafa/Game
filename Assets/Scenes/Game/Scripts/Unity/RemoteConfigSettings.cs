using System.Threading.Tasks;
using Unity.Services.Authentication;
using Unity.Services.Core;
using Unity.Services.RemoteConfig;
using UnityEngine;

namespace Scenes.Game.Scripts.Unity
{
    public class RemoteConfigSettings : MonoBehaviour
    {
        
        public static RemoteConfigSettings Instance { get; private set; }

        public int season = 0;

        public struct UserAttributes
        {
        }
        
        public struct AppAttributes
        {
            
        }

        async Task InitializeRemoteConfigAsync()
        {
            await UnityServices.InitializeAsync();

            if (!AuthenticationService.Instance.IsSignedIn)
            {
                await AuthenticationService.Instance.SignInAnonymouslyAsync();
            }
        }

        async void Start()
        {
            if (Utilities.CheckForInternetConnection())
            {
                await InitializeRemoteConfigAsync();
            }

            UserAttributes userAttributes = new UserAttributes();
            AppAttributes appAttributes = new AppAttributes();
            
            RemoteConfigService.Instance.FetchCompleted += InstanceOnFetchCompleted;
            RemoteConfigService.Instance.FetchConfigs(userAttributes, appAttributes);
            
        }

        private void InstanceOnFetchCompleted(ConfigResponse configResponse)
        {
            switch (configResponse.requestOrigin)
            {
                case ConfigOrigin.Remote:
                    season = RemoteConfigService.Instance.appConfig.GetInt("season");
                    break;
            }
        }

        private void Awake()
        {
            Instance = this;
        }
    }
}