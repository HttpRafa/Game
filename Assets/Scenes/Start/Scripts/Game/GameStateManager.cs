using System;
using UnityEngine;

namespace Scenes.Start.Scripts.Game
{
    
    public class GameStateManager : MonoBehaviour
    {

        private GameState _gameState = GameState.None;
        
        private void Awake()
        {
            DontDestroyOnLoad(gameObject);
        }

    }
    
}