namespace Entity
{
    public interface IDamageable
    {

        void OnDamage(DamageCause damageCause, float damage);

    }
}