using System.Collections;
using Entity.Player;
using UnityEngine;

namespace Entity.Bullet
{
    public class Bullet : MonoBehaviour
    {
        
        [SerializeField] private new Rigidbody rigidbody;

        private float _damage;
        private bool _owner;

        public void Setup(bool owner, Vector3 direction, float bulletSpeed, float damage)
        {
            _damage = damage;
            _owner = owner;

            // Apply force
            rigidbody.AddForce(direction * bulletSpeed, ForceMode.Impulse);

            StartCoroutine(DestroyAfterTime(5));
        }

        private void OnTriggerEnter(Collider other)
        {
            if (_owner)
            {
                PlayerController hitController = other.GetComponent<PlayerController>();
                if (hitController != null && hitController.IsLocal) return;
                
                Target target = other.GetComponent<Target>();
                if (target != null)
                {
                    target.Hit(DamageCause.Bullet, _damage);
                    Destroy(gameObject);
                }
                else
                {
                    Destroy(gameObject);
                }
            }
            else
            {
                Destroy(gameObject);
            }
        }

        IEnumerator DestroyAfterTime(float time)
        {
            yield return new WaitForSeconds(time);
            
            Destroy(gameObject);
        }
        
    }
    
}