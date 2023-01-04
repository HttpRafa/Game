using System;
using System.Collections.Generic;
using Scenes.Global.Scripts.Stats;
using TMPro;
using UnityEngine;

namespace Scenes.Game.Scripts.Hud
{
    public class ClientStatsHud : MonoBehaviour
    {
        
        [Serializable]
        public struct StatsDisplay
        {

            public TextMeshProUGUI textObject;
            public DisplayInfo displayInfo;

        }
        
        public enum DisplayInfo
        {
            FPS
        }
        
        [SerializeField] private List<StatsDisplay> statsDisplays;

        private FPSCounter _fpsCounter;
        private bool _fpsEnabled;

        private void Start()
        {
            if (FPSCounter.Instance != null)
            {
                _fpsCounter = FPSCounter.Instance;
                _fpsEnabled = true;
            }
        }

        private void FixedUpdate()
        {
            foreach (StatsDisplay statsDisplay in statsDisplays)
            {
                if (_fpsEnabled && statsDisplay.displayInfo == DisplayInfo.FPS)
                {
                    statsDisplay.textObject.text = _fpsCounter.FPS + " fps";
                }
            }
        }
        
    }
}