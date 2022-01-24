namespace Rudim.CLI
{
    internal interface ICliCommand
    {
        void Run(string[] parameters);
    }
}