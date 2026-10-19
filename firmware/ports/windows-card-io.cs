using System;
using System.Runtime.InteropServices;
using System.ComponentModel;
using System.Security.Cryptography;
public static class CardDirectRead {
 [DllImport("kernel32.dll",CharSet=CharSet.Unicode,SetLastError=true)] static extern IntPtr CreateFile(string p,uint a,uint s,IntPtr sec,uint c,uint flags,IntPtr t);
 [DllImport("kernel32.dll",SetLastError=true)] static extern bool ReadFile(IntPtr h,IntPtr b,uint n,out uint read,IntPtr o);
 [DllImport("kernel32.dll")] static extern bool CloseHandle(IntPtr h);
 [DllImport("kernel32.dll",SetLastError=true)] static extern IntPtr VirtualAlloc(IntPtr p,UIntPtr n,uint type,uint protect);
 [DllImport("kernel32.dll")] static extern bool VirtualFree(IntPtr p,UIntPtr n,uint type);
 public static string Hash(string path) {
  IntPtr h=CreateFile(path,0x80000000,3,IntPtr.Zero,3,0x20000000,IntPtr.Zero);
  if(h==new IntPtr(-1))throw new Win32Exception(Marshal.GetLastWin32Error());
  int size=4*1024*1024;IntPtr mem=VirtualAlloc(IntPtr.Zero,(UIntPtr)size,0x3000,4);
  if(mem==IntPtr.Zero){CloseHandle(h);throw new Win32Exception(Marshal.GetLastWin32Error());}
  try {using(var sha=SHA256.Create()) {byte[] bytes=new byte[size];uint got;
   while(true){if(!ReadFile(h,mem,(uint)size,out got,IntPtr.Zero))throw new Win32Exception(Marshal.GetLastWin32Error());if(got==0)break;Marshal.Copy(mem,bytes,0,(int)got);sha.TransformBlock(bytes,0,(int)got,bytes,0);if(got<size)break;}
   sha.TransformFinalBlock(new byte[0],0,0);return BitConverter.ToString(sha.Hash).Replace("-","").ToLowerInvariant();
  }} finally {VirtualFree(mem,UIntPtr.Zero,0x8000);CloseHandle(h);}
 }
}
