using System;
using System.Collections.Generic;
using Logger;
using Unity.Netcode;
using UnityEngine;
using UnityEngine.InputSystem;
using Weapon;

namespace Entity.Player
{

    public class PlayerController : NetworkBehaviour, IDamageable
    {
        public readonly NetworkVariable<float> NetHealth = new(writePerm: NetworkVariableWritePermission.Server);
        
        [Header("Player Object")]
        [SerializeField] private GameObject playerObject;
        [SerializeField] private CharacterController playerController;
        [SerializeField] private Transform physicsCheck;
        [SerializeField] private WeaponAim weaponAim;

        [Header("Indicator")]
        [SerializeField] private GameObject crosshairObject;
        [SerializeField] private GameObject rotationIndicator;
        
        [Header("Rendering")]
        [SerializeField] private new Camera camera;

        [Header("Network")]
        [SerializeField] private List<Component> componentsToDestroy;
        [SerializeField] private List<GameObject> objectsToDestroy;

        [Header("Ground")]
        [SerializeField] private LayerMask groundLayer;
        [SerializeField] private float groundDistance = 0.4f;
        
        [Header("Settings")]
        [SerializeField] private float speed;
        [SerializeField] private float rotationSpeed = 0.25f;
        [SerializeField] private float gravity = -9.81f;
        [SerializeField] private float maxHealth = 100f;

        public bool IsUsingMouse { get; private set; }
        public bool IsLocal { get; private set; }
        public Vector3 RotationTarget { get; private set; }
        
        private bool _isGrounded;
        private bool _ready;

        private Vector2 _movement;
        private Vector2 _mouseLook;
        private Vector2 _gamepadLook;

        private Vector3 _velocity;

        public void OnMove(InputAction.CallbackContext context)
        {
            _movement = context.ReadValue<Vector2>();
        }

        public void OnMouseLook(InputAction.CallbackContext context)
        {
            if (!IsUsingMouse) IsUsingMouse = true;
            _mouseLook = context.ReadValue<Vector2>();
        }

        public void OnGamepadLook(InputAction.CallbackContext context)
        {
            if (IsUsingMouse) IsUsingMouse = false;
            _gamepadLook = context.ReadValue<Vector2>();
        }

        public override void OnNetworkDespawn()
        {
            if (IsOwner)
            {
                LocalPlayer.Instance.Disable();   
            }
        }

        public override void OnNetworkSpawn()
        {
            if (IsServer)
            {
                NetHealth.Value = maxHealth;
            }
            
            if(IsOwner)
            {
                IsLocal = true;
                
                LocalPlayer localPlayer = LocalPlayer.Instance;
                localPlayer.Activate();
                localPlayer.BindCamera(gameObject);

                crosshairObject = localPlayer.AimCrosshair;
                camera = localPlayer.PlayerCamera;

                gameObject.tag = "Local Player";

                _ready = true;
                GameLogger.Info("PlayerController ready");
            }
            else
            {
                IsLocal = false;
            }
        }
        
        public void OnDamage(DamageCause damageCause, float damage)
        {
            HandleDamageServerRpc(damageCause, damage);
        }

        [ServerRpc]
        private void HandleDamageServerRpc(DamageCause damageCause, float damage)
        {
            float health = NetHealth.Value;
            if ((health - damage) <= 0)
            {
                // Kill Player
                NetHealth.Value = 0;
            }
            else
            {
                NetHealth.Value = health - damage;
            }
        }

        private void Start()
        {
            Cursor.visible = false;
            Cursor.lockState = CursorLockMode.Confined;
            
            rotationIndicator.SetActive(true);
        }

        private void FixedUpdate()
        {
            if (!IsOwner && IsSpawned)
            {
                foreach (var component in componentsToDestroy)
                {
                    Destroy(component);
                }
                foreach (var destroyObject in objectsToDestroy)
                {
                    Destroy(destroyObject);
                }
            }
            
            if(!_ready) return;
            
            if (crosshairObject.activeSelf && !IsUsingMouse)
            {
                crosshairObject.SetActive(false);
                weaponAim.Disable();
            } else if (!crosshairObject.activeSelf && IsUsingMouse)
            {
                crosshairObject.SetActive(true);
            }
        }

        private void Update()
        {
            if(!_ready) return;
            
            UpdateState();
            
            RotatePlayer();
            MovePlayer();
        }

        private void UpdateState()
        {
            _isGrounded = Physics.CheckSphere(physicsCheck.position, groundDistance, groundLayer);
        }

        private void RotatePlayer()
        {
            if (IsUsingMouse)
            {
                if (Physics.Raycast(camera.ScreenPointToRay(_mouseLook), out var raycastHit, 100f, groundLayer))
                {
                    RotationTarget = raycastHit.point;
                    weaponAim.TargetLocation = RotationTarget;
                    crosshairObject.transform.SetPositionAndRotation(RotationTarget, crosshairObject.transform.rotation);
                }
                
                // Rotate Player
                var lookPosition = RotationTarget - playerObject.transform.position;
                lookPosition.y = 0;
                var rotation = Quaternion.LookRotation(lookPosition);

                Vector3 aimDirection = new Vector3(RotationTarget.x, 0f, RotationTarget.z);
                if (aimDirection != Vector3.zero)
                {
                    playerObject.transform.rotation = Quaternion.Slerp(playerObject.transform.rotation, rotation, rotationSpeed);
                }
            }
            else
            {
                if (_gamepadLook.x != 0 && _gamepadLook.y != 0)
                {
                    Vector3 aimDirection = new Vector3(_gamepadLook.x, 0f, _gamepadLook.y);
                    if (aimDirection != Vector3.zero)
                    {
                        playerObject.transform.rotation = Quaternion.Slerp(playerObject.transform.rotation, Quaternion.LookRotation(aimDirection), rotationSpeed);
                    }
                }
            }
        }
        
        private void MovePlayer()
        {
            if (_isGrounded && _velocity.y < 0)
            {
                _velocity.y = -2f;
            }

            Vector3 movement = new Vector3(_movement.x, 0f, _movement.y);
            playerController.Move(movement * (speed * Time.deltaTime));

            _velocity.y += gravity * Time.deltaTime;
            playerController.Move(_velocity * Time.deltaTime);
        }
        
    }
}