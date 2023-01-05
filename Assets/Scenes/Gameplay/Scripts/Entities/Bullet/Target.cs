using Scenes.Gameplay.Scripts.Enums;
using UnityEngine;

namespace Scenes.Gameplay.Scripts.Entities.Bullet
{
    public class Target : MonoBehaviour
    {

        [SerializeField] private Component component;
        
        private IDamageable _damageable;

        private void OnValidate()
        {
            if (!(component is IDamageable))
            {
                component = null;
            }
        }

        private void Awake()
        {
            if (component != null) _damageable = (IDamageable)component;
        }

        public void Hit(DamageCause cause, float damage)
        {
            _damageable?.OnDamage(cause, damage);
        }
        
    }
}