using UnityEngine;

namespace Logger
{
    public class GameLogger
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
        
    }
}