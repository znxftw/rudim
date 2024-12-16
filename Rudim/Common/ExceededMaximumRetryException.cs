using System;
using System.Diagnostics.CodeAnalysis;
using System.Runtime.Serialization;

namespace Rudim.Common
{
    [ExcludeFromCodeCoverage]
    internal class ExceededMaximumRetryException(string message) : Exception(message);
}