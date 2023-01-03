using System.Collections.Generic;
using Scenes.Game.Scripts.Entities.Player.Logic;
using Unity.Netcode;
using UnityEngine;
using UnityEngine.InputSystem;

namespace Scenes.Game.Scripts.Entities.Player.Weapon
{
    public class WeaponSystem : NetworkBehaviour
    {
        private readonly NetworkVariable<int> _netSelectedWeapon = new(writePerm: NetworkVariableWritePermission.Server);

        [Header("Data")]
        [SerializeField] private Bullet.Bullet bulletPrefab;
        [SerializeField] private PlayerController playerController;

        [Header("Weapons")]
        [SerializeField] private List<Weapon> weapons;
        [SerializeField] private List<int> enabledWeapons;
        
        private Weapon _selectedWeapon;
        
        private bool _enabled;
        private bool _shoot;

        public void OnShoot(InputAction.CallbackContext context)
        {
            if (context.performed) _shoot = true;
            if (context.canceled) _shoot = false;
        }

        private void OnValueChanged(int previousValue, int newValue)
        {
            _selectedWeapon.gameObject.SetActive(false);
            _selectedWeapon = weapons[newValue];
            _selectedWeapon.gameObject.SetActive(true);
        }
        
        private void Start()
        {
            _netSelectedWeapon.OnValueChanged += OnValueChanged;
            
            if (enabledWeapons.Count > 0 && weapons.Count > 0)
            {
                _enabled = true;
                _selectedWeapon = weapons[_netSelectedWeapon.Value];
                _selectedWeapon.gameObject.SetActive(true);
            }
        }

        private void FixedUpdate()
        {
            _selectedWeapon.Tick();
            
            if (_shoot && _selectedWeapon.CanShoot())
            {
                _selectedWeapon.Shoot();
                if (playerController.IsUsingMouse)
                {
                    Vector3 muzzlePosition = _selectedWeapon.MuzzleTransform.position;
                    Vector3 shootDirection = ((playerController.RotationTarget + Vector3.up) - muzzlePosition).normalized;
                    FireShoot(_selectedWeapon, muzzlePosition, shootDirection);
                }
                else
                {
                    FireShoot(_selectedWeapon, _selectedWeapon.MuzzleTransform.position, transform.rotation * Vector3.forward);
                }
            }
        }
        
        private void FireShoot(Weapon weapon, Vector3 startPosition, Vector3 direction)
        {
            SpawnBullet(true, NetworkObjectId, startPosition, direction, weapon.WeaponData.bulletSpeed, weapon.WeaponData.damage);
            SpawnBulletServerRpc(NetworkObjectId, startPosition, direction, weapon.WeaponData.bulletSpeed, weapon.WeaponData.damage);
        }

        [ServerRpc]
        private void SpawnBulletServerRpc(ulong ownerId, Vector3 startPosition, Vector3 direction, float bulletSpeed, float damage)
        {
            SpawnBulletClientRpc(ownerId, startPosition, direction, bulletSpeed, damage);
        }

        [ClientRpc]
        private void SpawnBulletClientRpc(ulong ownerId, Vector3 startPosition, Vector3 direction, float bulletSpeed, float damage)
        {
            if (!IsOwner)
            {
                SpawnBullet(false, ownerId, startPosition, direction, bulletSpeed, damage);
            }
        }

        private void SpawnBullet(bool owner, ulong ownerId, Vector3 startPosition, Vector3 direction, float bulletSpeed, float damage)
        {
            Bullet.Bullet bullet = Instantiate(bulletPrefab, startPosition, Quaternion.identity);
            bullet.Setup(owner, ownerId, direction, bulletSpeed, damage);
        }

        public void ChangeWeapon(int index)
        {
            if (_enabled && (weapons.Count - 1) >= index && enabledWeapons.Contains(index) && _netSelectedWeapon.Value != index)
            {
                ChangeWeaponServerRpc(index);
            }
        }

        [ServerRpc]
        private void ChangeWeaponServerRpc(int index)
        {
            if ((weapons.Count - 1) >= index && enabledWeapons.Contains(index) && _netSelectedWeapon.Value != index)
            {
                _netSelectedWeapon.Value = index;
            }
        }

        private void OnDrawGizmos()
        {
            if (_enabled)
            {
                Vector3 target = (playerController.RotationTarget + Vector3.up);
                
                Gizmos.DrawSphere(target, 0.15f);
                var position = _selectedWeapon.MuzzleTransform.position;
                Debug.DrawLine(position, target, Color.red);
                Debug.DrawRay(position, transform.rotation * Vector3.forward, Color.green);
            }
        }
    }
}