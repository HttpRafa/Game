using UnityEngine;

namespace Scenes.Global.Scripts.Stats
{
    public class FPSCounter : MonoBehaviour
    {

        public static FPSCounter Instance { get; private set; }
        
        public int FPS { get; private set; }
        
        [SerializeField] private float updateInterval = 0.5f;

        private float _accum;
        private int _frames;
        private float _timeLeft;

        private void Awake()
        {
            DontDestroyOnLoad(this);
            
            Instance = this;
        }

        void Start()
        {
            _timeLeft = updateInterval;
        }
        
        void Update()
        {
            _timeLeft -= Time.deltaTime;
            _accum += Time.timeScale / Time.deltaTime;
            ++_frames;
            
            if (_timeLeft <= 0.0)
            {
                FPS = Mathf.RoundToInt(_accum / _frames);
                _timeLeft = updateInterval;
                _accum = 0.0f;
                _frames = 0;
            }
        }
        
    }
}