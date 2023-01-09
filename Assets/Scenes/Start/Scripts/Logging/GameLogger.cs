using UnityEngine;

namespace Scenes.Start.Scripts.Logging
{
    public abstract class GameLogger
    {

        public static void Info(object o)
        {
            Debug.Log("[Game Logger] " + o);
        }
        
        public static void Warning(object o)
        {
            Debug.LogWarning("[Game Logger] " + o);
        }
        
        public static void Error(object o)
        {
            Debug.LogError("[Game Logger] " + o);
        }

        public static void ErrorFormat(string format, params object[] args)
        {
            Debug.LogErrorFormat("[Game Logger] " + format, args);
        }

        public static void Format(string format, params object[] args)
        {
            Debug.LogFormat("[Game Logger] " + format, args);
        }
        
    }
}