using System.Collections.Generic;
using UnityEngine;

namespace Scenes.Gameplay.Scripts.Manager
{
    public class GameManager : MonoBehaviour
    {

        public static GameManager Instance { get; private set; }
        
        public GameMode GameMode { get; private set; }
        
        [SerializeField] private List<GameMode> gameModes = new();

        private void Awake()
        {
            Instance = this;
        }
        
    }
}