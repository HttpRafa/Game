namespace Scenes.Gameplay.Scripts.Enums
{
    public interface IDamageable
    {

        void OnDamage(DamageCause damageCause, float damage);

    }
}