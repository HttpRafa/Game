using System;
using System.Collections;
using Effects;
using Entity.Player;
using UnityEngine;

namespace Entity.Bullet
{
    
    [RequireComponent(typeof(Rigidbody))]
    public class Bullet : MonoBehaviour
    {

        [SerializeField] private Explosion explosion;
        
        [SerializeField] private int destroyTime = 5;

        private float _damage;
        private bool _owner;

        public void Setup(bool owner, Vector3 direction, float bulletSpeed, float damage)
        {
            _damage = damage;
            _owner = owner;

            // Apply force
            GetComponent<Rigidbody>().AddForce(direction * bulletSpeed, ForceMode.Impulse);

            StartCoroutine(DestroyAfterTime(destroyTime));
        }

        private void OnCollisionEnter(Collision collision)
        {
            ContactPoint contactPoint = collision.GetContact(0);
            if (_owner)
            {
                PlayerController hitController = collision.collider.GetComponent<PlayerController>();
                if (hitController != null && hitController.IsLocal) return;
                
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
            Explosion explosionObject = Instantiate(explosion, position, Quaternion.identity);
            if(forward != Vector3.zero) explosionObject.transform.forward = forward;
            
            Destroy(gameObject);
        }

        IEnumerator DestroyAfterTime(float time)
        {
            yield return new WaitForSeconds(time);
            
            BulletHit(transform.position, Vector3.zero);
        }
        
    }
    
}