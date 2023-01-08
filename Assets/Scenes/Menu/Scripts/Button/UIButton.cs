using TMPro;
using UnityEngine;
using UnityEngine.EventSystems;
using UnityEngine.TextCore.Text;
using FontStyles = TMPro.FontStyles;

namespace Scenes.Menu.Scripts.Button
{
    public class UIButton : MonoBehaviour, IPointerEnterHandler, IPointerExitHandler, IPointerClickHandler
    {

        [Header("Text")]
        [SerializeField] private TMP_Text textObject;
        [SerializeField] private FontStyles normalTextStyle;
        [SerializeField] private FontStyles hoverTextStyle;
        [SerializeField] private Color normalColor;
        [SerializeField] private Color hoverColor;

        [Header("Sound")]
        [SerializeField] private AudioSource audioSource;
        [SerializeField] private AudioClip hoverSound;
        [SerializeField] private AudioClip clickSound;
        
        public void OnPointerEnter(PointerEventData eventData)
        {
            textObject.color = hoverColor;
            textObject.fontStyle = hoverTextStyle;
            audioSource.PlayOneShot(hoverSound);
        }

        public void OnPointerExit(PointerEventData eventData)
        {
            textObject.color = normalColor;
            textObject.fontStyle = normalTextStyle;
        }

        public void OnPointerClick(PointerEventData eventData)
        {
            audioSource.PlayOneShot(clickSound);
        }
    }
}