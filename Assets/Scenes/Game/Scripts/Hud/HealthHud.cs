using System.Collections;
using Scenes.Game.Scripts.Entities.Player.Logic;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

namespace Scenes.Game.Scripts.Hud
{
    public class HealthHud : MonoBehaviour
    {

        [Header("Health Display")]
        [SerializeField] private Image healthImage;
        [SerializeField] private TextMeshProUGUI healthText;

        [Header("Damage Effect")] 
        [SerializeField] private Image damageEffect;
        //[SerializeField] private float damageEffectTime = 0.1f;

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
            if ((previousValue > newValue) && newValue > 0)
            {
                // TODO: Play damage animation
            }
            
            float progress = newValue / _maxHealth;
            healthImage.fillAmount = progress;
            healthText.text = newValue + "";
        }

    }
}