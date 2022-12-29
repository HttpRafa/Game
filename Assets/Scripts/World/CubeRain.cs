using System;
using UnityEngine;
using Random = System.Random;

namespace World
{
    public class CubeRain : MonoBehaviour
    {

        public GameObject prefab;
        public float spawnSpeed = 20f;

        public Vector3 spawnPosition;
        public Vector3 maxRandom;

        private float _time = 0f;
        private Random _random = new Random();
        
        private void Update()
        {
            _time += Time.deltaTime;
            if (_time >= spawnSpeed)
            {
                _time = 0;
                float randomValue = -((float)_random.NextDouble());
                if (_random.NextDouble() > 0.5f)
                {
                    randomValue = -randomValue;
                }

                Vector3 randomOffset = maxRandom;
                randomOffset.x *= randomValue;
                randomOffset.z *= -randomValue;

                Instantiate(prefab, spawnPosition + randomOffset, Quaternion.identity);
            }
        }
        
    }
}