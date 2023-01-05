using System;
using UnityEngine;

namespace Scenes.Start.Scripts.Game
{
    
    public class GameStateManager : MonoBehaviour
    {
        private void Awake()
        {
            DontDestroyOnLoad(gameObject);
        }
        
    }
    
}