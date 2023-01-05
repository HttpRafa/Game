using UnityEngine;

namespace Scenes.Gameplay.Scripts.Entities.Player.Weapon
{
    [CreateAssetMenu(fileName = "WeaponData", menuName = "Weapon/WeaponData", order = 0)]
    public class WeaponData : ScriptableObject
    {

        [Header("Info")]
        public new string name;

        [Header("Bullet")]
        public float damage;
        public float bulletSpeed;

        [Header("Reloading")]
        public int fireRate;
        public int magSize;
        public float reloadTime;

    }
}