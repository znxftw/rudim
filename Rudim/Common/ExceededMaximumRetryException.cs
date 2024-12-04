using System;
using System.Runtime.Serialization;

namespace Rudim.Common
{
    internal class ExceededMaximumRetryException(string message) : Exception(message);
}