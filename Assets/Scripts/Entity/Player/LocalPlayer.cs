using Cinemachine;
using Logger;
using UnityEngine;

namespace Entity.Player
{
    public class LocalPlayer : MonoBehaviour
    {

        private static LocalPlayer _instance;
        public static LocalPlayer Instance => _instance;

        [field:SerializeField] public GameObject AimCrosshair { get; private set; }
        [field:SerializeField] public Camera PlayerCamera { get; private set; }
        
        [SerializeField] private CinemachineVirtualCamera virtualCamera;

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

    }
}