using UnityEngine;

namespace Entity.Player
{
    public class PlayerInterface : MonoBehaviour
    {

        [SerializeField] private PlayerController playerController;

        private void Start()
        {
            playerController.NetHealth.OnValueChanged += OnHealthChanged;
        }

        private void OnHealthChanged(float previousValue, float newValue)
        {
            
        }

        private void Update()
        {
            
        }

    }
}