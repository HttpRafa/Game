using System.Collections.Generic;
using Scenes.Gameplay.Scripts.Entities.Player.Logic;
using Scenes.Gameplay.Scripts.Enums;
using UnityEngine;

namespace Scenes.Gameplay.Scripts.Manager
{
    public abstract class GameMode : MonoBehaviour
    {

        [SerializeField] protected new string name;
        
        private readonly List<GameObject> _gameModeObjects = new();

        private void OnPlayerDamage(DamageCause damageCause, float damage)
        {
            
        }
        
        private void OnPlayerDeath(PlayerController playerController)
        {
            
        }

        private void OnPlayerJoin(PlayerController playerController)
        {
            
        }

        private void OnPlayerLeave(PlayerController playerController)
        {
            
        }
        
        private void OnDestroy()
        {
            CleanUp();
        }

        private GameObject SpawnGameModeObject(GameObject prefab, Vector3 position, Quaternion rotation)
        {
            GameObject gameModeObject = Instantiate(prefab, position, rotation);
            _gameModeObjects.Add(gameModeObject);
            return gameModeObject;
        }
        
        private void CleanUp()
        {
            foreach (GameObject gameModeObject in _gameModeObjects)
            {
                Destroy(gameModeObject);
            }
            _gameModeObjects.Clear();
        }

    }
}