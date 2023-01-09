
using TMPro;
using UnityEngine;

namespace Scenes.Menu.Scripts.Menu
{
    public class VersionText : MonoBehaviour
    {

        [SerializeField] private TMP_Text text;
        
        private void Start()
        {
            text.text = Application.version + "e" + Application.unityVersion;
        }
        
    }
}
