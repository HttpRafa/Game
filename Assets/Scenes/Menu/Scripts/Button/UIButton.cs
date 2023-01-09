using TMPro;
using UnityEngine;
using UnityEngine.EventSystems;

namespace Scenes.Menu.Scripts.Button
{
    public class UIButton : MonoBehaviour, IPointerEnterHandler, IPointerExitHandler, ISelectHandler, IDeselectHandler
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

        private bool _selected = false;
        
        public void OnClick()
        {
            audioSource.PlayOneShot(clickSound);
        }

        public void OnSelect(BaseEventData eventData)
        {
            textObject.color = hoverColor;
            textObject.fontStyle = hoverTextStyle;
            audioSource.PlayOneShot(hoverSound);

            _selected = true;
        }

        public void OnDeselect(BaseEventData eventData)
        {
            textObject.color = normalColor;
            textObject.fontStyle = normalTextStyle;

            _selected = false;
        }

        public void OnPointerEnter(PointerEventData eventData)
        {
            textObject.color = hoverColor;
            audioSource.PlayOneShot(hoverSound);
        }

        public void OnPointerExit(PointerEventData eventData)
        {
            if (!_selected)
            {
                textObject.color = normalColor;   
            }
        }
    }
}