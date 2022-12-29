using System;
using System.Collections.Generic;
using UnityEngine;

namespace Weapon
{
    public class WeaponHolder : MonoBehaviour
    {

        public List<Weapon> weapons;
        public List<int> enabledWeapons;

        public int selectedWeapon = 0;
        private Weapon _selectedWeapon;
        
        private bool _enabled = false;
        
        private void Start()
        {
            if (enabledWeapons.Count > 0 && weapons.Count > 0)
            {
                _enabled = true;
                _selectedWeapon = weapons[selectedWeapon];
            }
        }

        private void Update()
        {
            if (_enabled)
            {
                
            }
        }

        public void ChangeWeapon(int index)
        {
            if (_enabled && (weapons.Count - 1) >= index && enabledWeapons.Contains(index) && selectedWeapon != index)
            {
                selectedWeapon = index;
                _selectedWeapon = weapons[selectedWeapon];
            }
        }

    }
}