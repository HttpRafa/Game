using System;
using Modules.Submodules.Pools.Scripts;
using UnityEngine;
using Random = UnityEngine.Random;

namespace Scenes.Menu.Scripts.Bullet
{
    public class MenuBulletSpawner : MonoBehaviour
    {
        
        [Header("Delay")]
        [SerializeField] private float randomStart = 1f;
        [SerializeField] private float randomEnd = 3f;
        
        [Header("Spawner")]
        [SerializeField] private Transform muzzleTransform;
        
        [Header("Bullet")]
        [SerializeField] private GameObject bulletPrefab;
        [SerializeField] private float bulletSpeed = 1f;

        private float _timeBetweenShoots;
        private float _timeSinceLastShoot;

        private void FixedUpdate()
        {
            _timeSinceLastShoot += Time.fixedDeltaTime;

            if (_timeSinceLastShoot > _timeBetweenShoots)
            {
                _timeBetweenShoots = Random.Range(randomStart, randomEnd);
                _timeSinceLastShoot = 0f;

                var bullet = PoolManager.Instance.GetFromPool<Gameplay.Scripts.Entities.Bullet.Bullet>();
                if (bullet != null)
                {
                    bullet.transform.position = muzzleTransform.position;
                    bullet.Setup(true, 0, transform.rotation * Vector3.forward, bulletSpeed, 0);
                }
            }
        }

    }
}