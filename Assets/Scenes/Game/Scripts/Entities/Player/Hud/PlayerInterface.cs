using Scenes.Game.Scripts.Entities.Player.Logic;
using UnityEngine;

namespace Scenes.Game.Scripts.Entities.Player.Hud
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