using Scenes.Game.Scripts.Entities.Player.Logic;
using TMPro;
using Unity.Netcode;
using Unity.Netcode.Transports.UTP;
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
        [SerializeField] private float damageEffectMaxAlpha = 0.35f;
        [SerializeField] private float damageEffectSpeed = 0.025f;

        private float _maxHealth;
        
        private float _damageEffectTargetAlpha = 0f;

        public void Setup(PlayerController playerController)
        {
            _maxHealth = playerController.NetMaxHealth.Value;
            
            playerController.NetHealth.OnValueChanged += OnHealthChanged;

            playerController.NetMaxHealth.OnValueChanged += OnMaxHealthChanged;
        }

        private void Update()
        {
            var imageColor = damageEffect.color;
            imageColor.a = Mathf.Lerp(imageColor.a, _damageEffectTargetAlpha, damageEffectSpeed);
            damageEffect.color = imageColor;

            if (imageColor.a >= _damageEffectTargetAlpha - 0.05f)
            {
                _damageEffectTargetAlpha = 0f;
            }
            
            
        }

        private void OnMaxHealthChanged(float previousValue, float newValue)
        {
            _maxHealth = newValue;
        }

        private void OnHealthChanged(float previousValue, float newValue)
        {
            if ((previousValue > newValue))
            {
                _damageEffectTargetAlpha = damageEffectMaxAlpha;
            }
            
            float progress = newValue / _maxHealth;
            healthImage.fillAmount = progress;
            healthText.text = newValue + "";
        }

    }
}