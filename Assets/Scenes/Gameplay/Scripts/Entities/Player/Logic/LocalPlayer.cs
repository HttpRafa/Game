using Cinemachine;
using Scenes.Gameplay.Scripts.Hud;
using Scenes.Start.Scripts.Logging;
using UnityEngine;

namespace Scenes.Gameplay.Scripts.Entities.Player.Logic
{
    public class LocalPlayer : MonoBehaviour
    {

        private static LocalPlayer _instance;
        public static LocalPlayer Instance => _instance;

        [field:SerializeField] public GameObject AimCrosshair { get; private set; }
        [field:SerializeField] public Camera PlayerCamera { get; private set; }
        
        [SerializeField] private CinemachineVirtualCamera virtualCamera;
        
        [Header("Hud")]
        [SerializeField] private HealthHud healthHud;

        private void Awake()
        {
            _instance = this;
        }

        public void Activate()
        {
            PlayerCamera.gameObject.SetActive(true);
            GameLogger.Info("Activated LocalPlayer");
        }

        public void Disable()
        {
            PlayerCamera.gameObject.SetActive(false);
            GameLogger.Info("Disabled LocalPlayer");
        }
        
        public void BindCamera(GameObject playerObject)
        {
            virtualCamera.Follow = playerObject.transform;
            virtualCamera.LookAt = playerObject.transform;
            
            GameLogger.Info("Updated VirtualCamera target");
        }

        public void SetupHud(PlayerController playerController)
        {
            healthHud.Setup(playerController);
        }
        
    }
}