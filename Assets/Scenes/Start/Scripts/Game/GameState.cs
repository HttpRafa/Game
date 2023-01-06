using System.Collections.Generic;
using System.Threading.Tasks;
using Scenes.Start.Scripts.Logging;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace Scenes.Start.Scripts.Game
{
    
    [System.Serializable]
    public class GameState
    {
        
        [field:SerializeField] public GameStateType Type { get; private set; }
        [field:SerializeField] public List<int> Scenes { get; private set; }

        public enum GameStateType
        {
            Menu,
            Gameplay
        }

        public async Task UnLoad()
        {
            List<AsyncOperation> asyncOperations = new List<AsyncOperation>();
            
            foreach (var scene in Scenes)
            {
                asyncOperations.Add(SceneManager.UnloadSceneAsync(scene));
            }
            
            GameLogger.Info("Unloading " + asyncOperations.Count + " scenes for gameState of type " + Type);
            while (asyncOperations.Count > 0)
            {
                asyncOperations.RemoveAll(operation => operation.isDone);
                await Task.Yield();
            }
            GameLogger.Info("Unloaded gameState of type " + Type);
        }

        public async Task Load()
        {
            List<AsyncOperation> asyncOperations = new List<AsyncOperation>();

            foreach (var scene in Scenes)
            {
                asyncOperations.Add(SceneManager.LoadSceneAsync(scene, LoadSceneMode.Additive));
            }

            GameLogger.Info("Loading " + asyncOperations.Count + " scenes for gameState of type " + Type);
            while (asyncOperations.Count > 0)
            {
                asyncOperations.RemoveAll(operation => operation.isDone);
                await Task.Yield();
            }
            GameLogger.Info("Loaded gameState of type " + Type);
        }
    }
}