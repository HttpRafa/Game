using UnityEngine;

namespace Weapon
{
    public class WeaponAim : MonoBehaviour
    {

        public Vector3 TargetLocation { private get; set; }
        
        private void Update()
        {
            if (TargetLocation != Vector3.zero)
            {
                transform.LookAt(TargetLocation);
            }
        }

        public void Disable()
        {
            TargetLocation = Vector3.zero;
        }
        
    }
}