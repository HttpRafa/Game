using System.Collections;
using Modules.Submodules.Pools.Scripts;
using Scenes.Gameplay.Scripts.Entities.Player.Logic;
using Scenes.Gameplay.Scripts.Enums;
using UnityEngine;

namespace Scenes.Gameplay.Scripts.Entities.Bullet
{
    
    [RequireComponent(typeof(Rigidbody))]
    public class Bullet : MonoBehaviour, IPoolObject
    {

        [Header("Reset Info")]
        [SerializeField] private Rigidbody bulletRigidbody;
        [SerializeField] private TrailRenderer trailRenderer;
        
        [SerializeField] private int destroyTime = 5;

        private float _damage;
        private bool _owner;
        private ulong _ownerId;

        public void Setup(bool owner, ulong ownerId, Vector3 direction, float bulletSpeed, float damage)
        {
            _damage = damage;
            _owner = owner;
            _ownerId = ownerId;

            // Apply force
            bulletRigidbody.AddForce(direction * bulletSpeed, ForceMode.Impulse);

            // Clear TrailRender
            trailRenderer.Clear();
            
            StartCoroutine(DestroyAfterTime(destroyTime));
        }

        private void OnCollisionEnter(Collision collision)
        {
            if(collision.collider.GetComponent<Bullet>() != null) return;
            PlayerController hitController = collision.collider.GetComponent<PlayerController>();
            if (hitController != null && hitController.NetworkObjectId == _ownerId) return;

            ContactPoint contactPoint = collision.GetContact(0);
            if (_owner)
            {
                Target target = collision.collider.GetComponent<Target>();
                if (target != null)
                {
                    target.Hit(DamageCause.Bullet, _damage);
                    BulletHit(contactPoint.point, contactPoint.normal);
                }
                else
                {
                    BulletHit(contactPoint.point, contactPoint.normal);
                }
            }
            else
            {
                BulletHit(contactPoint.point, contactPoint.normal);
            }
        }

        private void BulletHit(Vector3 position, Vector3 forward)
        {
            var hitObject = PoolManager.Instance.GetFromPool<BulletHit>();
            if (hitObject != null)
            {
                hitObject.transform.position = position;
                if(forward != Vector3.zero) hitObject.transform.forward = forward;
                hitObject.Play();   
            }

            PoolManager.Instance.TakeToPool<Bullet>(this);
        }

        IEnumerator DestroyAfterTime(float time)
        {
            yield return new WaitForSeconds(time);
            
            BulletHit(transform.position, Vector3.zero);
        }

        public void OnCreatedInPool()
        {
            
        }

        public void OnGettingFromPool()
        {
            bulletRigidbody.velocity = Vector3.zero;
        }
        
    }
    
}