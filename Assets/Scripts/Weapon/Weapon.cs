using Objects;
using UnityEngine;

namespace Weapon
{
    public class Weapon : MonoBehaviour
    {
        
        [field:SerializeField] public Transform MuzzleTransform { get; private set; }
        [field:SerializeField] public WeaponData WeaponData { get; private set; }
        
        public float TimeSinceLastShoot { get; private set; }

        public void Tick()
        {
            TimeSinceLastShoot += Time.fixedDeltaTime;
        }

        public bool CanShoot()
        {
            return TimeSinceLastShoot >= (1f / (WeaponData.fireRate / 60f));
        }

        public void Shoot()
        {
            TimeSinceLastShoot = 0f;
        }
    }
}