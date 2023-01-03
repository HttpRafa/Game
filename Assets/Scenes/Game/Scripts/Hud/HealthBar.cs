using Scenes.Game.Scripts.Entities.Player.Logic;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

namespace Scenes.Game.Scripts.Hud
{
    public class HealthBar : MonoBehaviour
    {

        [SerializeField] private Image healthImage;
        [SerializeField] private TextMeshProUGUI healthText;
        
        private float _maxHealth;

        public void Setup(PlayerController playerController)
        {
            _maxHealth = playerController.NetMaxHealth.Value;
            
            playerController.NetHealth.OnValueChanged += OnHealthChanged;
            playerController.NetMaxHealth.OnValueChanged += OnMaxHealthChanged;
        }

        private void OnMaxHealthChanged(float previousValue, float newValue)
        {
            _maxHealth = newValue;
        }

        private void OnHealthChanged(float previousValue, float newValue)
        {
            float progress = newValue / _maxHealth;
            healthImage.fillAmount = progress;
            healthText.text = newValue + "";
        }
        
    }
}