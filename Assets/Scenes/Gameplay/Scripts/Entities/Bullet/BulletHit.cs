using System.Collections.Generic;
using Modules.Submodules.Pools.Scripts;
using UnityEngine;

namespace Scenes.Gameplay.Scripts.Entities.Bullet
{
    public class BulletHit : MonoBehaviour, IPoolObject
    {

        [SerializeField] private List<ParticleSystem> particleSystems;

        public void Play()
        {
            foreach (ParticleSystem system in particleSystems)
            {
                system.Play();
            }
        }

        private void OnParticleSystemStopped()
        {
            PoolManager.Instance.TakeToPool<BulletHit>(this);
        }

        public void OnCreatedInPool()
        {
            
        }

        public void OnGettingFromPool()
        {
            
        }
        
    }
}